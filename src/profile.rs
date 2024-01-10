use chrono::*;
use itertools::Itertools;
use once_cell::sync::Lazy;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    rc::Rc, io::stdout, default,
};

use crate::Errors;
use reqwest::{self, Client};
use scraper::{html::Html, Selector};

static TITLE_SELECTOR: Lazy<Result<Selector, Errors>> =
    Lazy::new(|| Selector::parse("title").map_err(|_| Errors::SelectorGenerateError));

type URL<'a> = &'a str;
type Title<'a> = &'a str;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct URLTitlePair<'a> {
    pub url: URL<'a>,
    pub title: Title<'a>,
    
    pub is_highlighted: bool,
    pub t_created: NaiveDateTime,
}

impl<'a> URLTitlePair<'a> {
    pub fn new(url: URL<'a>, title: Title<'a>) -> Self {
        Self {
            url,
            title,
            is_highlighted: false,
            t_created: Utc::now().naive_utc()
        }
    }
}

impl<'a> Default for URLTitlePair<'a> {
    fn default() -> Self {
        Self::new("https://duckduckgo.com/", "Duckduckgo Homepage")
    }
}



/** The mode by which a lookup is performed.
 *
 * The bool value is for blurry search (TRUE if blurry, FALSE if exact).
 * By default, it will be by title AND blurry. */
pub enum SearchMode {
    ByURL(bool),
    ByTitle(bool),
    EitherMatch(bool),
}

impl Default for SearchMode {
    fn default() -> Self {
        Self::ByTitle(true)
    }
}

#[derive(Default, PartialEq, Eq)]
pub enum SortMode {
    ByTitle,
    ByTitleRev,
    ByURL,
    ByURLRev,
    #[default] ByDateCreation,
    ByDateCreationRev
}


#[allow(dead_code)]
pub struct Profile<'a> {
    id: usize,
    pub name: &'a str,

    pub t_created: NaiveDateTime,
    pub t_last_visited: NaiveDateTime,

    pub pairs: Vec<Rc<URLTitlePair<'a>>>,
    // pairs: HashSet<Rc<URLTitlePair<'a>>>,

    pub curr_sort_mode: SortMode,

    urls: HashMap<URL<'a>, HashSet<Rc<URLTitlePair<'a>>>>, // NOTE: Multiple URLs can have the same title (given or self-designated); same for titles
    titles: HashMap<Title<'a>, HashSet<Rc<URLTitlePair<'a>>>>,
}




impl<'a> Profile<'a> {
    pub fn new(last_id: usize, name: &str) -> Self {
        let t_created = Utc::now().naive_utc();

        Self {
            id: last_id + 1,
            name,

            t_created,
            t_last_visited: t_created,

            pairs: Vec::new(),
            // pair: HashSet::new(),

            curr_sort_mode: SortMode::default(),

            urls: HashMap::new(),
            titles: HashMap::new(),
        }
    }

    pub fn new_with_info(
        last_id: usize,
        name: &str,
        t_created: NaiveDateTime,
        pairs: Vec<Rc<URLTitlePair<'_>>>
    ) -> Self {
        Self {
            id: last_id,
            name,
            t_created,
            // TODO:
            t_last_visited: t_created,
            pairs,
            curr_sort_mode: SortMode::default(),
            urls: todo!(),
            titles: todo!(),
        }
    }

    /**
     * Add a new URL-Title Pair to the current profile.
     * Note:
     *  - if there is no user-given title AND fetching the page's own title failed,
     *  - OR if an *exact* pair with the same URL AND title already exists in this profile,
     *  - the pair will NOT be added.
     * */

    pub fn add_new(&mut self, url: &str, given_title: Option<&str>) -> Result<(), Errors> {
        let title = given_title
            .or_else(|| 
                // TODO:
                // Profile::get_title(url)?
                Some("Title not given")
            )
            .ok_or(Errors::ParseTitleError)?;

        let pair = Rc::new(URLTitlePair::new(url, title));

        // if !self.pairs.insert(pair.clone()) {
        //     return Err(Errors::PairAlreadyExistsError);
        // }

        self.urls
            .entry(url)
            .and_modify(|by_url| {
                by_url.insert(pair.clone());
            })
            .or_insert({
                HashSet::from([pair.clone()])
            });

        self.titles
            .entry(title)
            .and_modify(|by_title| {
                by_title.insert(pair.clone());
            })
            .or_insert({
                HashSet::from([pair.clone()])
            });
        Ok(())
    }

    /** Behavior: if fetching failed, do not change previous title */
    pub fn refresh_get_titles(&mut self) {
        self.urls.iter_mut().for_each(|(url, title)| {
            // TODO:
            // if let Ok(new_title) = Profile::get_title(url) {
            //     *title = new_title;
            // }
        });
    }

    /** Search by the given searchand in this profile. */
    pub fn search(
        &self,
        searchand: &str,
        mode: SearchMode,
    ) -> Result<Vec<&Rc<URLTitlePair>>, Errors> {
        match mode {
            SearchMode::ByURL(blurry) => Profile::search_by_url(&self, searchand, blurry),
            SearchMode::ByTitle(blurry) => Profile::search_by_title(&self, searchand, blurry),
            SearchMode::EitherMatch(blurry) => {
                let by_url = Profile::search_by_url(&self, searchand, blurry);
                let by_title = Profile::search_by_title(&self, searchand, blurry);

                if by_title.is_err() && by_url.is_err() {
                    return Err(Errors::NothingFoundError);
                }

                todo!()
            }
        }
    }

    fn search_by_url(
        &self,
        searchand: &str,
        is_blurry: bool,
    ) -> Result<Vec<&Rc<URLTitlePair>>, Errors> {
        let res = if is_blurry {
            self.urls
                .iter()
                .filter_map(|(&url, pairs)| {
                    if !url.contains(searchand) {
                        None
                    } else {
                        Some(pairs.into_iter().collect_vec())
                    }
                })
                .flatten()
                .collect_vec()
        } else {
            self.urls
                .get(searchand)
                .ok_or(Errors::NothingFoundError)?
                .into_iter()
                .collect_vec()
        };

        match res.is_empty() {
            true => Ok(res),
            false => Err(Errors::NothingFoundError),
        }
    }
    fn search_by_title(
        &self,
        searchand: &str,
        is_blurry: bool,
    ) -> Result<Vec<&Rc<URLTitlePair>>, Errors> {
        todo!()
    }

    fn remove_pair(&mut self, removend: &Rc<URLTitlePair>) -> Result<(), Errors> {
        if self.urls.remove(&removend.url).is_some() && self.titles.remove(&removend.title).is_some() {
            // self.pairs.remove(removend);
            self.pairs.remove(idx);

            Ok(())
        } else {
            Err(Errors::LookupDeletionFailedError)
        }
    }

    async fn get_title(url: &str) -> Result<String, Errors> {
        let response = Client::new()
            .get(url)
            .send()
            .await
            .map_err(|_| Errors::RequestGetError)?;

        let body = response.text().await.map_err(|_| Errors::ParseTextError)?;
        let doc = Html::parse_document(&body);

        // NOTE: by this point, the main program should have confirmed that TITLE_SELECTOR is
        // generated successfully

        let title = doc
            .select(TITLE_SELECTOR.as_ref().map_err(|e| *e)?)
            .next()
            .ok_or(Errors::ParseTitleError)?
            .text()
            .collect_vec()
            .join("");

        Ok(title)
    }
}
