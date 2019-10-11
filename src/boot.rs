use std::io;

use tmux_copy::tmux;

fn main() -> Result<(), io::Error> {
    tmux::spawn()
}
