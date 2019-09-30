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

/// Set foreground color to red.
pub const RED: &str = "\x1B[38;5;9m";

/// Set foreground color to green.
pub const GREEN: &str = "\x1B[38;5;10m";

impl<'s> fmt::Display for find::Match<'s> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "\x1B[{};{}H", self.row + 1, self.col + 1)
    }
}
