/// Clear the screen.
pub const CLEAR: &str = "\x1B[2J";

/// Switch to alternate screen buffer.
pub const ALT: &str = "\x1B[?1049h"; 

/// Switch to main screen buffer.
pub const MAIN: &str = "\x1B[?1049l";

/// Hide cursor.
pub const HIDE: &str = "\x1B[?25l";

/// Show cursor.
pub const SHOW: &str = "\x1B[?25h";

/// Reset foreground style.
pub const RESET: &str = "\x1B[39m";

/// Set foreground color to red.
pub const COLOR: &str = "\x1B[38;5;1";
