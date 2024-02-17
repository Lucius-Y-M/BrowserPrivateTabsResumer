use std::io::Stdout;
use crossterm::{cursor::MoveTo, style::{Print, Color, SetBackgroundColor, SetForegroundColor, ResetColor}, terminal::{Clear, ClearType}};
use crate::{Profile, Errors, write_stdout};




#[allow(unused_macros)]
macro_rules! format_pair {
    ($pair: ident) => {
        {
            let url = $pair.url
                // .get_mut()
                // .unwrap_or(&mut String::from("### FETCH FAILED"))
                .clone();
            let title = $pair.title
                // .get_mut()
                // .unwrap_or(&mut String::from("### FETCH FAILED"))
                .clone();

            String::from_iter([">> Title: ", &title, " | URL: ", &url])
        }
    };
}
macro_rules! format_profile {
    ($prfl: ident) => {
        {
            let name = &$prfl.get_name();
            let len = $prfl.get_pairs().len();
            let t_last = $prfl.get_time_last_visited();
            format!(">> {} | {} | {}", name, len, t_last)
        }
    };
}







const SEP: &'static str = "======================================";

const STATIC_INFO_MAINMENU: [&'static str; 11] = [
    "=== Firefox Tab Resumer ===",
    "Author: Lucius Y. Men, Written in Rust",
    "Latest Version: v0.1, Updated: 2 Jan 2024",
    SEP,
    ">> Below is the list of all existing profiles.",
    ">> Use UP / DOWN ARROWS to navigate and select a profile",
    ">> Press I to INITIALIZE (start) the highlighted profile",
    ">> Press E to EDIT the profile,",
    ">> Press D to DELETE the profile",
    ">> If you want a new profile, press N to enter its name",
    SEP,
];
pub const STATIC_INFO_MAINMENU_LEN: u16 = STATIC_INFO_MAINMENU.len() as u16;





pub const COLOR_FG_DECLARE: Color = Color::Green;
pub const COLOR_FG_DEFAULT: Color = Color::White;
pub const COLOR_FG_HILIT: Color = Color::Cyan;

pub const COLOR_BG_HILIT: Color = Color::White;




pub fn render_line(stdout: &mut Stdout, line: &str, opt_fg_color: Option<Color>) -> Result<(), Errors> {
    
    if let Some(color) = opt_fg_color {
        write_stdout!(
            stdout,
            SetForegroundColor(color)
        )?;
    }
    write_stdout!(
        stdout,
        Print(line)
    )?;
    Ok(())
}



pub fn render_beginning(stdout: &mut Stdout) -> Result<(), Errors> {
    write_stdout!(
        stdout,

        Clear(ClearType::All), // this is necessary

        MoveTo(0, 0),
        SetForegroundColor(COLOR_FG_DECLARE),
        Print(STATIC_INFO_MAINMENU.join("\n\r")),
        MoveTo(0, STATIC_INFO_MAINMENU_LEN + 1),
        ResetColor
    )?;

    Ok(())
}







pub fn render_list_of_profiles(
    stdout: &mut Stdout,
    prfls: &Vec<Profile>,
    pos_row_last: u16,
    pos_col: u16,

    highlight_idx: usize
) -> Result<(), Errors> {

    let pos_row = pos_row_last
        .checked_add(1)
        .ok_or(Errors::CursorPosOverflowError)?;

    write_stdout!(
        stdout,
        MoveTo(pos_col, pos_row)
    )?;

    for (idx, prfl) in prfls.iter().enumerate() {
        
        let printstr = format_profile!(prfl);

        if idx == highlight_idx {
            write_stdout!(
                stdout,
                SetForegroundColor(COLOR_FG_HILIT),
                SetBackgroundColor(COLOR_BG_HILIT)
            )?;
        }

        write_stdout!(
            stdout,
            Print(printstr),
            ResetColor
        )?;
    }

    Ok(())
}



pub fn render_profile(
    stdout: &mut Stdout,
    prfl: &mut Profile,
    pos_row_last: u16, /* the last row BEFORE we start rendering */
    pos_col: u16
) -> Result<(), Errors>
{

    let pos_row = pos_row_last
        .checked_add(1)
        .ok_or(Errors::CursorPosOverflowError)?;

    // move cursor to position, then render
    write_stdout!(
        stdout,
        MoveTo(
            pos_col,
            pos_row
        )
    )?;

    render_profile_impl(stdout, prfl)
}





fn render_profile_impl(stdout: &mut Stdout, prfl: &mut Profile) -> Result<(), Errors> {
    

    for pair in prfl.get_pairs().iter() {

        let mut fg_color = COLOR_FG_DEFAULT;

        if pair.clone().is_highlighted() {
            fg_color = COLOR_FG_HILIT;
            write_stdout!(stdout, SetBackgroundColor(COLOR_BG_HILIT))?;
        }
        
        write_stdout!(
            stdout,
            SetForegroundColor(fg_color),
            Print(format_pair!(pair)),
            ResetColor
        )?;
    }

    Ok(())
}

