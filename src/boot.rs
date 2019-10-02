use std::env;
use std::fs;
use std::io;
use std::os::unix::net;
use std::path;

use tmux_copy::tmux;

/// Destructor deletes leftover Unix domain socket file.
struct Bomb<'path>(&'path path::Path);

impl<'path> Drop for Bomb<'path> {
    fn drop(&mut self) {
        fs::remove_file(&self.0).ok();
    }
}

fn main() -> Result<(), io::Error> {
    // Get unique fingerprint for window and socket naming
    let uuid = uuid::Uuid::new_v4()
        .to_simple()
        .to_string();

    // Set up socket path
    let path = env::temp_dir().join(format!("tmux-copy-{}", uuid));
    let bomb = Bomb(&path);

    // Listen on socket
    let sock = net::UnixDatagram::bind(&path)?;

    // Pass active pane ID and socket path to `main` binary
    let from = tmux::active()?;
    let to = tmux::spawn(&from, &path, &uuid)?;

    // Wait for `main` to find matches and paint the canvas before swapping
    let mut buffer = [0; 1];
    let _ = sock.recv(&mut buffer)?;
    match buffer[0] {
    | 0 => tmux::swap(Some(&from), &to)?,
    | _ => println!("No matches found."),
    }

    // Make sure `bomb`'s lifetime extends to end of function
    Ok(drop(bomb))
}
