use std::error;

use tmux_copy::tmux;

fn main() -> Result<(), Box<dyn error::Error + Send + Sync>> {
    tmux::spawn()?;
    Ok(())
}
