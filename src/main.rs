#![feature(const_trait_impl)]


use std::io::stdout;

use firefox_resumer::{*, io::read_profiles};

use crossterm::{self, cursor::MoveTo, event::{self, Event, KeyCode}, style::ResetColor, terminal::{self, Clear, ClearType}};
use once_cell::sync::Lazy;




static MOVEMENTS: Lazy<[Event; 2]> = Lazy::new(|| {
    [Event::Key(KeyCode::Up.into()), Event::Key(KeyCode::Down.into())]
});


fn main() -> Result<(), Errors> {
    main_impl()?;

    // test();
    Ok(())
}

fn test() {
    let s = chrono::Utc::now().naive_utc().to_string();
    let want = s
        .split_once(".")
        .unwrap().0
        .to_string()
        .replace(":", "::")
        .replace(" ", "::")
        .replace("-", "::");
    
    println!("{}", want);
}



enum WhatToRender<'a> {
    ListProfiles,
    Profile(&'a Profile),

    BrowserTab,
}

fn main_impl() -> Result<(), Errors> {
    let mut stdout = stdout();

    /* clear everything */
    write_stdout!(
        stdout,
        Clear(ClearType::Purge)
    )?;
    


    let _raw = terminal::enable_raw_mode();
    let profiles = read_profiles();


    let prfls = profiles.unwrap_or_default();

    // if prfls.is_empty() {
    //     render_line(
    //         &mut stdout,
    //         " == You currently do not have any profiles, or the reading failed for some reason.",
    //         Some(COLOR_FG_DECLARE)
    //     )?;
    // } else {
    //     render_list_of_profiles(&mut stdout, &prfls, STATIC_INFO_MAINMENU_LEN, 0, 0)?;
    // }


    /* event loop */

    let mut highlight_idx: Option<usize> = None;

    let mut curr_prfl_idx: Option<usize> = None;

    let mut profile_count = 0usize;
    let mut render_what = WhatToRender::ListProfiles;
    loop {

        let event = event::read().map_err(|_| Errors::EventReadFailedError)?;

        // we are currently in the MAIN MENU (choose / add / delete profiles)
        match render_what {
            WhatToRender::ListProfiles => {
                render_beginning(&mut stdout)?;

                if prfls.is_empty() {
                    render_line(
                        &mut stdout,
                        " == You currently do not have any profiles, or the reading failed for some reason.",
                        Some(COLOR_FG_DECLARE)
                    )?;
                } else {
                    if highlight_idx.is_none() { highlight_idx = Some(0usize); }
                    render_list_of_profiles(&mut stdout, &prfls, STATIC_INFO_MAINMENU_LEN, 0, highlight_idx.clone().unwrap())?;
                    profile_count = prfls.len();
                }
        
                match event {
                    /* ESC */
                    _ if event == Event::Key(KeyCode::Esc.into()) => {
                        write_stdout!(
                            stdout,
                            Clear(ClearType::All),
            
                            MoveTo(0, 0),
                            ResetColor
                        )?;
                        break;
                    }
                    
                    
                    /* UP AND DOWN */
                    _ if event == Event::Key(KeyCode::Up.into()) => {
                        if let Some(ref mut idx) = highlight_idx {
                            *idx = match idx.checked_sub(1) {
                                Some(idx) => idx,
                                None => profile_count - 1,
                            };
        
                        }
                    }
                    _ if event == Event::Key(KeyCode::Down.into()) => {
                        if let Some(ref mut idx) = highlight_idx {
                            match *idx + 1 < profile_count {
                                true => {
                                    *idx += 1;
                                },
                                false => {
                                    *idx = 0;
                                },
                            }
        
                        }
                    }
        
                    /* SELECTING CURR PROFILE */
                    _ if event == Event::Key(KeyCode::Enter.into()) => {
                        if let Some(ref idx) = highlight_idx {
                            if let Some(prfl) = prfls.get(*idx) {
                                render_what = WhatToRender::Profile(prfl);
                                highlight_idx = match prfl.get_pairs().len() > 0 {
                                    true => Some(0),
                                    false => None,
                                };
                            }
                        }
                    }
        
        
                    /* EVERYTHING ELSE: === DO NOTHING === */
                    _ => {       
                    }
                }
            },
            WhatToRender::Profile(prfl) => {
                render_one_profile(&mut stdout, prfl, STATIC_INFO_MAINMENU_LEN, 0, highlight_idx)?;

                match event {
                    /* ESC -> go back to previous level */
                    _ if event == Event::Key(KeyCode::Esc.into()) => {
                        render_what = WhatToRender::ListProfiles;
                        highlight_idx = curr_prfl_idx;
                    }
                    
                    
                    /* UP AND DOWN */
                    _ if event == Event::Key(KeyCode::Up.into()) => {
                        if let Some(ref mut idx) = highlight_idx {
                            *idx = match idx.checked_sub(1) {
                                Some(idx) => idx,
                                None => profile_count - 1,
                            };
        
                        }
                    }
                    _ if event == Event::Key(KeyCode::Down.into()) => {
                        if let Some(ref mut idx) = highlight_idx {
                            match *idx + 1 < profile_count {
                                true => {
                                    *idx += 1;
                                },
                                false => {
                                    *idx = 0;
                                },
                            }
        
                        }
                    }

                    _ => {}
                }

                
            },
            WhatToRender::BrowserTab => {

            },
        }
        
        
        
    }
    Ok(())
}
