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
    let uuid = uuid::Uuid::new_v4().to_simple();
    let file = format!("tmux-copy-{}", uuid);
    let path = env::temp_dir().join(file);
    let bomb = Bomb(&path);
    let sock = net::UnixDatagram::bind(&path)?;
    let from = tmux::active()?;
    let to = tmux::spawn(&from, &path, &uuid.to_string())?;
    let mut buffer = [0; 1];
    let _ = sock.recv(&mut buffer)?;
    match buffer[0] {
    | 0 => tmux::swap(Some(&from), &to)?,
    | _ => println!("No matches found."),
    }
    Ok(drop(bomb))
}
