#![feature(const_trait_impl)]


use std::io::stdout;

use firefox_resumer::{*, io::read_profiles};

use crossterm::{self, cursor::MoveTo, event::{self, Event, KeyCode}, style::ResetColor, terminal::{self, Clear, ClearType}};
use once_cell::sync::Lazy;




static MOVEMENTS: Lazy<[Event; 2]> = Lazy::new(|| {
    [Event::Key(KeyCode::Up.into()), Event::Key(KeyCode::Down.into())]
});


fn main() -> Result<(), Errors> {
    // main_impl()

    test();
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


fn main_impl() -> Result<(), Errors> {
    let mut stdout = stdout();

    /* clear everything */
    // write_stdout!(
    //     stdout,
    //     Clear(ClearType::Purge)
    // )?;
    


    let _raw = terminal::enable_raw_mode();
    let profiles = read_profiles();


    let prfls = profiles.unwrap_or_default();
    if prfls.is_empty() {
        render_line(
            &mut stdout,
            " == You currently do not have any profiles, or the reading failed for some reason.",
            Some(COLOR_FG_DECLARE)
        )?;
    } else {
        render_list_of_profiles(&mut stdout, &prfls, STATIC_INFO_MAINMENU_LEN, 0, 0)?;
    }


    /* event loop */
    loop {
        let event = event::read().map_err(|_| Errors::EventReadFailedError)?;
        let mut is_changed = false;

        // we are currently in the MAIN MENU (choose / add / delete profiles)
        // render_beginning(&mut stdout)?;

        if prfls.is_empty() {
            // render_line(
            //     &mut stdout,
            //     " == You currently do not have any profiles, or the reading failed for some reason.",
            //     Some(COLOR_FG_DECLARE)
            // )?;
        } else {
            // render_list_of_profiles(&mut stdout, &prfls, STATIC_INFO_MAINMENU_LEN, 0, 0)?;
        }


        if event == Event::Key(KeyCode::Esc.into()) {
            write_stdout!(
                stdout,
                Clear(ClearType::All),

                MoveTo(0, 0),
                ResetColor
            )?;

            break;
        }

        if MOVEMENTS.contains(&event) {
            is_changed = true;
        }

        // if let Event::Key()

        if is_changed {
            // rerender

            is_changed = false;
        }
    }
    Ok(())
}
