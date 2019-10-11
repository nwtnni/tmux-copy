use std::io;

use tmux_copy::tmux;

fn main() -> Result<(), io::Error> {
    let active = std::env::args()
        .nth(1)
        .expect("Expected active pane ID");
    tmux::spawn(&active)
}
