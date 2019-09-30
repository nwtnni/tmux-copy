use std::fmt;

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

#[derive(Copy, Clone, Debug)]
pub struct Go(pub u16, pub u16);

impl fmt::Display for Go {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "\x1B[{};{}H", self.1 + 1, self.0 + 1)
    }
}
