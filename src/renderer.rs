use std::{rc::Rc, io::Stdout};
use crossterm::{style::{Print, Color, SetBackgroundColor, SetForegroundColor, ResetColor}, cursor::MoveTo};
use crate::{Profile, URLTitlePair, Errors, write_stdout, SortMode};




#[allow(unused_macros)]
macro_rules! format_pair {
    ($pair: ident) => {
        {
            let url = $pair.url;
            let title = $pair.title;
            String::from_iter([">> Title: ", title, " | URL: ", url].into_iter())
        }
    };
}
macro_rules! format_profile {
    ($prfl: ident) => {
        {
            let name = $prfl.name;
            let len = $prfl.pairs.len();
            let t_last = $prfl.t_last_visited;
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





const COLOR_FG_DECLARE: Color = Color::Green;
const COLOR_FG_DEFAULT: Color = Color::White;
const COLOR_FG_HILIT: Color = Color::Cyan;

const COLOR_BG_HILIT: Color = Color::White;




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
        MoveTo(0, 0),
        SetForegroundColor(COLOR_FG_DECLARE),
        Print(STATIC_INFO_MAINMENU.join("\n\r")),
        MoveTo(0, STATIC_INFO_MAINMENU_LEN + 1),
        ResetColor
    )?;

    Ok(())
}







pub fn render_list_of_profiles<'a>(
    stdout: &mut Stdout,
    prfls: &Vec<Profile<'a>>,
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
    prfl: &Profile,
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

    render_profile_impl(stdout, &prfl.pairs, prfl.curr_sort_mode)
}


pub fn change_sort_mode(prfl: &mut Profile, new_sort_mode: SortMode) {

    if prfl.curr_sort_mode != new_sort_mode {
        prfl.curr_sort_mode = new_sort_mode;

        match new_sort_mode {
            SortMode::ByTitle => prfl.pairs.sort_unstable_by(|a, b| a.title.cmp(b.title)),
            SortMode::ByTitleRev => prfl.pairs.sort_unstable_by(|a, b| b.title.cmp(a.title)),
            SortMode::ByURL => prfl.pairs.sort_unstable_by(|a, b| a.url.cmp(b.url)),
            SortMode::ByURLRev => prfl.pairs.sort_unstable_by(|a, b| b.url.cmp(a.url)),
            SortMode::ByDateCreation => prfl.pairs.sort_unstable_by(|a, b| a.t_created.cmp(&b.t_created)),
            SortMode::ByDateCreationRev => prfl.pairs.sort_unstable_by(|a, b| b.t_created.cmp(&a.t_created)),
        }
    }
}






fn render_profile_impl(stdout: &mut Stdout, list: &Vec<Rc<URLTitlePair>>, sort_mode: SortMode) -> Result<(), Errors> {    
    for pair in list.iter() {

        let mut fg_color = COLOR_FG_DEFAULT;

        if pair.is_highlighted {
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

