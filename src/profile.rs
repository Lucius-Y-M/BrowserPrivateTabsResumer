use chrono::*;
use itertools::Itertools;
use once_cell::sync::Lazy;
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::Errors;
use reqwest::{self, Client};
use scraper::{html::Html, Selector};

static TITLE_SELECTOR: Lazy<Result<Selector, Errors>> =
    Lazy::new(|| Selector::parse("title").map_err(|_| Errors::SelectorGenerateError));

type URL<'a> = &'a str;
type Title<'a> = &'a str;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct URLTitlePair<'a>(URL<'a>, Title<'a>);

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

#[allow(dead_code)]
pub struct Profile<'a> {
    id: usize,
    name: String,

    t_created: NaiveDateTime,
    t_last_modified: NaiveDateTime,

    pairs: HashSet<Rc<URLTitlePair<'a>>>,

    urls: HashMap<URL<'a>, HashSet<Rc<URLTitlePair<'a>>>>, // NOTE: Multiple URLs can have the same title (given or self-designated); same for titles
    titles: HashMap<Title<'a>, HashSet<Rc<URLTitlePair<'a>>>>,
}

impl<'a> Profile<'a> {
    pub fn new(last_id: usize, name: &str) -> Self {
        let t_created = Utc::now().naive_utc();

        Self {
            id: last_id + 1,
            name: name.to_owned(),

            t_created,
            t_last_modified: t_created,

            pairs: HashSet::new(),

            urls: HashMap::new(),
            titles: HashMap::new(),
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
            .or_else(|| Profile::get_title(url)?)
            .ok_or(Errors::ParseTitleError)?;

        let pair = Rc::new(URLTitlePair(url, title));

        if !self.pairs.insert(pair.clone()) {
            return Err(Errors::PairAlreadyExistsError);
        }

        self.urls
            .entry(url)
            .and_modify(|by_url| {
                by_url.insert(pair.clone());
            })
            .or_insert({
                let k = HashSet::new();
                k.insert(pair.clone());
                k
            });

        self.titles
            .entry(title)
            .and_modify(|by_title| {
                by_title.insert(pair.clone());
            })
            .or_insert({
                let k = HashSet::new();
                k.insert(pair.clone());
                k
            });
        Ok(())
    }

    /** Behavior: if fetching failed, do not change previous title */
    pub fn refresh_get_titles(&mut self) {
        self.urls.iter_mut().for_each(|(url, title)| {
            if let Ok(new_title) = Profile::get_title(url) {
                *title = new_title;
            }
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
        if self.urls.remove(&removend.0).is_some() && self.titles.remove(&removend.1).is_some() {
            self.pairs.remove(removend);

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
