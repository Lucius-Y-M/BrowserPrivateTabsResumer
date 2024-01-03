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
const STATIC_INFO_MAINMENU_LEN: usize = STATIC_INFO_MAINMENU.len();

#[allow(unused_macros)]
macro_rules! format_for_display {
    ($title: ident, $url: ident) => {
        String::from_iter([">> Title: ", $title, " | URL: ", $url].into_iter())
    };
}
