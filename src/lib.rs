pub mod errors;
pub mod renderer;
pub mod profile;
pub mod io;


pub use errors::*;
pub use renderer::*;
pub use profile::*;

use serde::Deserialize;
pub use crossterm::execute;




#[macro_export]
#[cfg(feature = "debug_print")]
macro_rules! debug_println {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!($($arg)*);
    }
}
#[macro_export]
#[cfg(not(feature = "debug_print"))]
macro_rules! debug_println {
    ($($arg:tt)*) => {}
}




#[macro_export]
macro_rules! write_stdout {
    ($stdout: ident, $($item: expr),+) => {
        crossterm::execute!(
            $stdout,
            $($item),+
        ).map_err(|_| Errors::WriteToStdoutError)
    };
}