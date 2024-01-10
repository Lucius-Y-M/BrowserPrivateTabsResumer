use std::{fs, rc::Rc};

use chrono::{NaiveDateTime, NaiveDate};
use itertools::Itertools;
use toml::Value;

use crate::{Profile, Errors, URLTitlePair};

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


const TOML_GEN_HEADER: &str = "General";
const TOML_GEN_PRFL_NAME: &str = "name";
const TOML_GEN_PRFL_ID: &str = "id";
const TOML_GEN_TIME_CR: &str = "time_created";


const TOML_OBJ_HEADER: &str = "BrowserTab";
const TOML_OBJ_URL: &str = "url";
const TOML_OBJ_TITLE: &str = "title";
const TOML_OBJ_TIME_CR: &str = "time_created";



const fn is_file_pattern_correct(file_name: &str) -> bool {
    file_name.starts_with(FILE_PREFIX)
    & file_name.ends_with(FILE_EXTSN) 
}



fn parse_toml(toml_file_name: &str) -> Option<Profile> {


    let toml_file = fs::read_to_string(toml_file_name).ok()?;


    // init Profile
    let gen_table = toml::from_str::<Value>(&toml_file)
        .ok()?
        .get(TOML_GEN_HEADER)?
        .as_table()?;

    let name = gen_table
        .get(TOML_GEN_PRFL_NAME)?
        .as_str()?;
    let last_id = gen_table
        .get(TOML_GEN_PRFL_ID)?
        .as_str()?
        .parse::<usize>()
        .ok()?;



    // toml format: yyyy:: mm:: dd:: hh:: mm:: ss
    let t_created: NaiveDateTime = {
        let mut time: Vec<u32> = gen_table
            .get(TOML_GEN_TIME_CR)?
            .as_str()?
            .split("::")
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
            )?
    };


    // read browser tabs
    let pairs =
        if let Some(arr) = toml::from_str::<Value>(
                &toml_file
            )
            .ok()?
            .get("BrowserTab")?
            .as_array()
        {
            let pairs = arr
                .into_iter()
                .filter_map(|v| {
                    let url = v.get("url")?.as_str()?;
                    let title = v.get("title")?.as_str()?;
                    Some(Rc::new(URLTitlePair::new(url, title)))
                })
                .collect_vec();

            Some(pairs)
        }
        
        else
        {
            None
        };
    
    if pairs.is_none() {
        Some(Profile::new(last_id, name))
    } else {
        Some(Profile::new_with_info(last_id, name, t_created, pairs?))
    }

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
pub fn read_profiles<'a>() -> Result<Vec<Profile<'a>>, Errors> {

    let profiles = fs::read_dir(".")
        .map_err(|_| Errors::FSReadError)?
        .filter_map(|file| {
            match file {
                Ok(file) => {
                    let file_name = file.file_name().to_str()?;
                    match is_file_pattern_correct(file_name) {
                        true => Some(file_name),
                        false => None,
                    }
                },
                Err(_) => None,
            }
        })
        .fold(Vec::with_capacity(DEFAULT_LIMIT), |mut acc, next_file_name| {
            if let Some(prfl) = parse_toml(next_file_name) {
                acc.push(prfl);
            };
            acc
        });

    if profiles.len() > 0 {
        Ok(profiles)
    } else {
        Err(Errors::NoTOMLFilesFoundError)
    }
}