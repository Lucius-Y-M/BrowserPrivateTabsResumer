

#![allow(unused_imports)]

use std::sync::{
    atomic::AtomicUsize,
    atomic::Ordering as AtomicOrdering,
    Arc,
    Mutex
};


use chrono;
use once_cell::sync::Lazy;


use crate::Errors;

/// The value is read from the TOML file, OR it is 0 by default
static PROFILE_ID_COUNTER: Lazy<AtomicUsize> = Lazy::new(|| {
    AtomicUsize::new(0)
});



#[derive(Debug, Default, PartialEq, Eq)]
pub enum SortMode {
    ByTitle,
    ByTitleRev,

    ByURL,
    ByURLRev,

    #[default] ByDateCreation,
    ByDateCreationRev
}


#[derive(Debug, Default, Clone)]
pub struct URLTitlePair {
    // pub url: Arc<Mutex<String>>,
    // pub title: Arc<Mutex<String>>,
    pub url: String,
    pub title: String,


    t_created: chrono::NaiveDateTime,

    is_highlighted: bool,
}

impl URLTitlePair {
    pub fn new(url: &str, title: &str) -> Self {
        Self {
            url: String::from(url),
            title: String::from(title),
            // url: Arc::new(Mutex::new(String::from(url))),
            // title: Arc::new(Mutex::new(String::from(title))),

            t_created: chrono::Utc::now().naive_utc(),

            is_highlighted: false,
        }
    }

    pub fn is_highlighted(self) -> bool {
        self.is_highlighted
    }
    pub fn set_highlighted(&mut self, is: bool) {
        self.is_highlighted = is;
    }

    pub fn update_url(&mut self, new_url: &str) -> Result<(), Errors> {
        self.url.clear();
        self.url.push_str(new_url);

        Ok(())

        // if let Ok(ref mut mutex) = self.url.try_lock() {
        //     **mutex = String::from(new_url);

        //     Ok(())
        // } else {
        //     Err(Errors::MutexLockFailedError)
        // }
    }

    pub fn update_title(&mut self, new_title: &str) -> Result<(), Errors> {
        self.title.clear();
        self.title.push_str(new_title);

        Ok(())
        // if let Ok(ref mut mutex) = self.title.try_lock() {
        //     **mutex = String::from(new_title);

        //     Ok(())
        // } else {
        //     Err(Errors::MutexLockFailedError)
        // }
    }
}


#[derive(Debug, Default)]
pub struct Profile {
    id: usize,
    name: String,

    pairs: Vec<URLTitlePair>,
    sort: SortMode,


    t_created: chrono::NaiveDateTime,
    t_last_modified: chrono::NaiveDateTime
}

impl Profile {
    pub fn builder() -> ProfileBuilder {
        ProfileBuilder::default()
    }

    pub fn get_sort_mode(self) -> SortMode {
        self.sort
    }
    pub fn change_sort_mode(&mut self, mode: SortMode) {
        if self.sort == mode { return; }
        
        self.sort = mode;

        match self.sort {
            SortMode::ByTitle => self.pairs.sort_unstable_by(|a, b| a.title.cmp(&b.title)),
            SortMode::ByTitleRev => self.pairs.sort_unstable_by(|a, b| b.title.cmp(&a.title)),
            SortMode::ByURL => self.pairs.sort_unstable_by(|a, b| a.url.cmp(&b.url)),
            SortMode::ByURLRev => self.pairs.sort_unstable_by(|a, b| b.url.cmp(&a.url)),
            SortMode::ByDateCreation => self.pairs.sort_unstable_by(|a, b| a.t_created.cmp(&b.t_created)),
            SortMode::ByDateCreationRev => self.pairs.sort_unstable_by(|a, b| b.t_created.cmp(&a.t_created)),
        }

        self.last_modified()
    }

    pub fn get_id(self) -> usize {
        self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn change_name(&mut self, new_name: &str) {
        self.name.clear();
        self.name.push_str(new_name);

        self.last_modified()
    }

    pub fn get_pairs(&self) -> &Vec<URLTitlePair> {
        &self.pairs
    }
    pub fn get_mut_pairs(&mut self) -> &mut Vec<URLTitlePair> {
        &mut self.pairs
    }


    pub fn get_time_created(&self) -> chrono::NaiveDateTime {
        self.t_created
    }
    pub fn get_time_last_visited(&self) -> chrono::NaiveDateTime {
        self.t_last_modified
    }

    #[inline(always)]
    fn last_modified(&mut self) {
        self.t_last_modified = chrono::Utc::now().naive_utc()
    }
}

#[derive(Debug)]
pub struct ProfileBuilder {
    id: usize,
    name: Option<String>,

    pairs: Option<Vec<URLTitlePair>>,
    sort: SortMode
}

impl Default for ProfileBuilder {
    fn default() -> Self {
        Self {
            id: PROFILE_ID_COUNTER.fetch_add(1, AtomicOrdering::SeqCst),
            
            name: None,
            pairs: None,
            sort: SortMode::default(),
        }
    }
}


impl ProfileBuilder {
    pub fn add_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());

        self
    }

    pub fn add_title_url_pair(mut self, url: &str, title: &str) -> Self {
        if let Some(ref mut pairs) = self.pairs {
            pairs.push(URLTitlePair::new(url, title));
        } else {
            self.pairs = Some(vec![ URLTitlePair::new(url, title) ]);
        }

        self
    }
    pub fn add_many_title_url_pairs(mut self, mut url_titles: Vec<URLTitlePair>) -> Self {
        if let Some(ref mut pairs) = self.pairs {
            pairs.append(&mut url_titles);
        } else {
            self.pairs = Some(url_titles);
        }
        
        self
    }

    pub fn set_sort_mode(mut self, mode: SortMode) -> Self {
        self.sort = mode;

        self
    }

    pub fn build(self) -> Profile {

        let t_created = chrono::Utc::now().naive_utc();

        
        Profile {
            id: self.id,
            name: self.name.unwrap_or(format!("# Unnamed Profile No. {}", self.id)),
            
            pairs: self.pairs.unwrap_or_default(),
            sort: self.sort,

            t_created,
            t_last_modified: t_created
        }
    }
}