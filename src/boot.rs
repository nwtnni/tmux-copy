use std::error;
use std::io::Read as _;
use std::net;

use clipboard::ClipboardProvider;

#[cfg(any(
    target_os = "windows",
    target_os = "macos",
))]
use clipboard::ClipboardContext;

#[cfg(any(
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "linux",
    target_os = "netbsd",
    target_os = "openbsd",
))]
use clipboard_ext::x11_bin::ClipboardContext;

#[cfg(any(
    target_os = "android",
    target_os = "ios",
))]
const _: () = compile_error!("Unsupported operating system");

use tmux_copy::PORT;
use tmux_copy::tmux;

fn main() -> Result<(), Box<dyn error::Error>> {
    tmux::spawn()?;

    let listener = net::TcpListener::bind((net::Ipv4Addr::LOCALHOST, PORT))?;
    let mut socket = listener.accept().map(|(socket, _)| socket)?;
    let mut buffer = String::new();

    match socket.read_to_string(&mut buffer)? {
    | 0 => (),
    | _ => ClipboardContext::new()?.set_contents(buffer)?,
    }

    Ok(())
}
