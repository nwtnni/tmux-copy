use std::io;

use tmux_copy::tmux;

fn main() -> Result<(), io::Error> {
    let from = tmux::active()?;
    let to = tmux::spawn(&from)?;
    tmux::swap(Some(&from), &to)?;
    Ok(())
}
