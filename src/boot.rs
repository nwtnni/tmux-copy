use std::io;
use std::net;

use tmux_copy::tmux;

fn main() -> Result<(), io::Error> {
    let bind = net::TcpListener::bind(("localhost", 0))?;
    let addr = bind.local_addr()?;
    let from = tmux::active()?;
    let to = tmux::spawn(&from, &addr)?;
    let _ = bind.accept()?;
    tmux::swap(Some(&from), &to)?;
    Ok(())
}
