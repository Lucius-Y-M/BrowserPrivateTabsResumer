pub mod errors;
pub mod renderer;
pub mod profile;
pub mod io;


pub use errors::*;
pub use renderer::*;
pub use profile::*;

pub use crossterm::execute;


#[macro_export]
macro_rules! write_stdout {
    ($stdout: ident, $($item: expr),+) => {
        crossterm::execute!(
            $stdout,
            $($item),+
        ).map_err(|_| Errors::WriteToStdoutError)
    };
}