use std::io;
use std::time;
use std::thread;

use tmux_copy::tmux;

fn main() -> Result<(), io::Error> {
    let from = tmux::active()?;
    let to = tmux::spawn(&from)?;
    // HACK:
    //
    // There's a race condition between `main` and `boot` here.
    // After `main` is spawned, it uses `tmux capture-pane` to
    // get the visible text in the active pane. However, if this next
    // `swap` call runs first, then the active pane will be swapped
    // into a full-size window, causing `capture-pane` to capture
    // too many rows.
    thread::sleep(time::Duration::from_millis(10));
    tmux::swap(Some(&from), &to)?;
    Ok(())
}
