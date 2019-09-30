use std::fmt;

use crate::find;

/// Clear the screen.
pub const CLEAR: &str = "\x1B[2J";

/// Hide cursor.
pub const HIDE: &str = "\x1B[?25l";

/// Show cursor.
pub const SHOW: &str = "\x1B[?25h";

/// Reset foreground style.
pub const RESET: &str = "\x1B[39m";

#[derive(Copy, Clone, Debug)]
pub struct Color(pub u8);

impl fmt::Display for Color {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "\x1b[38;5;{}m", self.0)
    }
}

impl<'s> fmt::Display for find::Match<'s> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "\x1B[{};{}H", self.row + 1, self.col + 1)
    }
}
