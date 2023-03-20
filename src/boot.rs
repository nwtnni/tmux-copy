use std::error;
use std::io::Read as _;
use std::net;

use tmux_copy::tmux;
use tmux_copy::PORT;

fn main() -> Result<(), Box<dyn error::Error + Send + Sync>> {
    tmux::spawn()?;

    let mut clipboard = copypasta_ext::try_context().expect("Failed to initialize clipboard");
    let listener = net::TcpListener::bind((net::Ipv4Addr::LOCALHOST, PORT))?;
    let mut socket = listener.accept().map(|(socket, _)| socket)?;
    let mut buffer = String::new();

    match socket.read_to_string(&mut buffer)? {
        0 => (),
        _ => clipboard.set_contents(buffer)?,
    }

    Ok(())
}
