use std::env;
use std::fs;
use std::io;
use std::os::unix::net;
use std::path;

use tmux_copy::tmux;

struct Bomb<'path>(&'path path::Path);

impl<'path> Drop for Bomb<'path> {
    fn drop(&mut self) {
        fs::remove_file(&self.0).ok();
    }
}

fn main() -> Result<(), io::Error> {
    let path = env::temp_dir().join("tmux-copy");
    let bomb = Bomb(&path);
    let sock = net::UnixDatagram::bind(&path)?;
    let from = tmux::active()?;
    let to = tmux::spawn(&from, &path)?;
    let mut buffer = [0; 1];
    let _ = sock.recv(&mut buffer)?;
    if buffer[0] == 0 {
        tmux::swap(Some(&from), &to)?;
    } else {
        println!("No matches found.")
    }
    Ok(drop(bomb))
}
