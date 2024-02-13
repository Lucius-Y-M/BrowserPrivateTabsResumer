#![feature(const_trait_impl)]


use std::io::stdout;

use firefox_resumer::{*, io::read_profiles};

use crossterm::{self, terminal, event::{self, Event, KeyCode}};
use once_cell::sync::Lazy;




static MOVEMENTS: Lazy<[Event; 2]> = Lazy::new(|| {
    [Event::Key(KeyCode::Up.into()), Event::Key(KeyCode::Down.into())]
});

fn main() -> Result<(), Errors> {
    let mut stdout = stdout();
    
    render_beginning(&mut stdout)?;

    let _raw = terminal::enable_raw_mode();
    loop {
        let event = event::read().map_err(|_| Errors::EventReadFailedError)?;
        let mut is_changed = false;
        let profiles = read_profiles();

        // we are currently in the MAIN MENU (choose / add / delete profiles)
        render_beginning(&mut stdout)?;

        if let Ok(prfls) = profiles {

            render_list_of_profiles(&mut stdout, &prfls, STATIC_INFO_MAINMENU_LEN, 0, 0)?;


        } else {
            render_line(
                &mut stdout,
                " == You currently do not have any profiles, or the reading failed for some reason.",
                Some(COLOR_FG_DECLARE)
            )?;
        }


        if event == Event::Key(KeyCode::Esc.into()) {
            break;
        }

        if MOVEMENTS.contains(&event) {
            is_changed = true;
        }

        // if let Event::Key()

        if is_changed {
            // rerender
        }
    }
    Ok(())
}
