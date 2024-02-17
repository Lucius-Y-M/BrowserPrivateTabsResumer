use std::fs;

use chrono::{NaiveDateTime, NaiveDate};
use itertools::Itertools;

use crate::{Profile, Errors, URLTitlePair, Deserialize};

///
///
///Saves PROFILES and reads from them
///
///
///
///

/// The default number of initial pre-allocation
/// for reading profile files.
/// Many people will have more than 30 browser tabs open at once,
/// but how many, if using this app,
/// will have more than 30 PROFILES at the same time?
/// 
/// Surely it cannot happen, right?
/// 
/// Right?
const DEFAULT_LIMIT: usize = 30;



const FILE_PREFIX: &str = "ITR_PRFL_";
const FILE_EXTSN: &str = ".toml";

const TIME_SEPARATOR: &str = "::";

#[allow(dead_code)] const TOML_GEN_HEADER: &str = "General";
#[allow(dead_code)] const TOML_GEN_PRFL_NAME: &str = "name";
#[allow(dead_code)] const TOML_GEN_PRFL_ID: &str = "id";
#[allow(dead_code)] const TOML_GEN_TIME_CR: &str = "time_created";


#[allow(dead_code)] const TOML_OBJ_HEADER: &str = "BrowserTab";
#[allow(dead_code)] const TOML_OBJ_URL: &str = "url";
#[allow(dead_code)] const TOML_OBJ_TITLE: &str = "title";
#[allow(dead_code)] const TOML_OBJ_TIME_CR: &str = "time_created";



fn is_file_pattern_correct(file_name: &str) -> bool {
    file_name.starts_with(FILE_PREFIX)
    & file_name.ends_with(FILE_EXTSN) 
}


// ============== FOR TOML PARSING



#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct Config {
    General: General,
    BrowserTab: Vec<BrowserTab>
}


#[derive(Debug, Deserialize)]
struct General {
    name: String,
    id: u16,
    time_created: String,
}
#[derive(Debug, Deserialize)]
struct BrowserTab {
    url: String,
    title: String,

    t_created: String
}



#[inline(always)]
fn parse_into_naivedatetime(t: String, separator: &str) -> Option<NaiveDateTime> {
    let mut time = t
        .split(separator)
        .filter_map(|s| s.parse::<u32>().ok())
        .collect_vec();
    time.reverse();


    if time.len() != 6 {
        return None;
    }

    NaiveDate::
        from_ymd_opt(
            time.pop()? as i32, 
            time.pop()?, 
            time.pop()?
        )?
        .and_hms_opt(
            time.pop()?, 
            time.pop()?,
            time.pop()?
        )
}


fn parse_toml(toml_file_name: &str) -> Option<Profile> {
    

    let toml_file = fs::read_to_string(toml_file_name).ok()?;
    println!("TOML FILE Opened.");

    let config: Config = toml::from_str(&toml_file).unwrap();
    println!("TOML FILE into str.");

    // init Profile
    let general = config.General;
    println!("TOML FILE into table.");

    let name = general.name;
    println!("TOML FILE name got: {}.", name);
    let last_id = general
        .id;
        // .get(TOML_GEN_PRFL_ID)?
        // .as_str()?
        // .parse::<usize>()
        // .ok()?;
    println!("TOML FILE id got: {}.", last_id);


    // toml format: yyyy:: mm:: dd:: hh:: mm:: ss
    let t_created: NaiveDateTime = parse_into_naivedatetime(general.time_created, TIME_SEPARATOR)?;
    println!("TOML FILE t created parsed: {}.", t_created);


    // read browser tabs
    let pairs = config
        .BrowserTab
        .into_iter()
        .filter_map(|tab| {
            let url = if tab.url.is_empty() {
                return None;
            } else {
                tab.url
            };

            let title = if tab.title.is_empty() {
                String::from("No Title Given")
            } else {
                tab.title
            };

            let t_created: NaiveDateTime = parse_into_naivedatetime(tab.t_created, TIME_SEPARATOR)?;

            Some(URLTitlePair::from_save(url, title, t_created))
        })
        .collect_vec();
    
    Some(
        Profile::builder()
            .add_name(&name)
            .add_many_title_url_pairs(pairs)
            .build()
        )
}





///
///Reads all profile files in the current folder for this pattern:
///```
/// file_name.starts_with("ITR_PRFL")
/// & file_name.ends_with(".toml")
///```
///and returns an [Ok(Vec<Profile>)] if any found,
///or [Err(())] if none exists
///(OR, rarely, if [```fs::read_dir(".")```] somehow fails).
///
#[inline(always)]
pub fn read_profiles() -> Result<Vec<Profile>, Errors> {

    let profiles = fs::read_dir(".")
        .map_err(|_| Errors::FSReadError)?
        .filter_map(|file| {
            match file {
                Ok(file) => {
                    
                    println!("FILE READ: {:?}", file);

                    let fname = file.file_name();
                    let file_name = fname.to_str()?;
                    match is_file_pattern_correct(file_name) {
                        true => {
                            parse_toml(file_name)
                        },
                        false => None,
                    }
                },
                Err(_) => {
                    println!("FILE FAILED TO READ");
                    None
                },
            }
        })
        .fold(Vec::with_capacity(DEFAULT_LIMIT), |mut acc, prfl| {
            acc.push(prfl);
            acc
        });

    if profiles.len() > 0 {
        Ok(profiles)
    } else {
        Err(Errors::NoTOMLFilesFoundError)
    }
}