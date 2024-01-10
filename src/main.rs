use std::io::stdout;

use firefox_resumer::{*, io::read_profiles};

use crossterm::{self, terminal, event::{self, Event, KeyCode}};




const MOVEMENTS: [Event; 2] = [Event::Key(KeyCode::Up.into()), Event::Key(KeyCode::Down.into())];

fn main() -> Result<()> {
    let mut stdout = stdout();
    
    render_beginning(&mut stdout);

    let _raw = terminal::enable_raw_mode();
    loop {
        let event = event::read()?;
        let mut is_changed = false;
        let profiles = read_profiles();

        // we are currently in the MAIN MENU (choose / add / delete profiles)
        render_beginning(&mut stdout);

        if profiles.is_err() {
            render_line(
                &mut stdout,
                " == You currently do not have any profiles, or the reading failed for some reason.",
                opt_fg_color
            )?;
        } else {
            render_list_of_profiles(&mut stdout, prfls, pos_row_last, pos_col, highlight_idx)?;
        }


        if event == Event::Key(KeyCode::Esc.into()) {
            break;
        }

        if MOVEMENTS.contains(&event) {
            is_changed = true;
        }

        // if let Event::Key()
    }
    Ok(())
}
