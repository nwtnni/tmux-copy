use std::error;
use std::env;
use std::io;
use std::io::Write;
use std::os::unix::net;

use clipboard::*;

use tmux_copy::ansi;
use tmux_copy::find;
use tmux_copy::hint;
use tmux_copy::term;
use tmux_copy::tmux;

struct Bomb<'pane>(&'pane str);

impl<'pane> Drop for Bomb<'pane> {
    fn drop(&mut self) {
        tmux::swap(None, &self.0).ok();
    }
}

/// Color of matching text
const FULL: ansi::Color = ansi::Color(6);

/// Color of unselected hint
const HINT: ansi::Color = ansi::Color(10);

/// Color of selected hint
const PICK: ansi::Color = ansi::Color(11);

fn main() -> Result<(), Box<dyn error::Error>> {

    let mut args = env::args().skip(1);

    let pane = args.next().expect("Missing active pane");
    let path = args.next().expect("Missing socket path");
    let sock = net::UnixDatagram::unbound()?;
    let bomb = Bomb(&pane);

    let text = tmux::capture_text(&pane)?;
    let show = tmux::capture_all(&pane)?;

    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut term = term::Term::new(&mut stdin, &mut stdout)?;

    let matches = find::matches(&text.trim_end());

    // Short-circuit
    if matches.is_empty() {
        sock.send_to(&[1], &path)?;
        return Ok(())
    }

    let mut hints = hint::hints(matches.len())
        .zip(&matches)
        .collect::<Vec<_>>();

    // Write out original text, matches, and hints
    write!(&mut term, "{}", show.trim_end())?;
    for (h, m) in &hints {
        write!(&mut term, "{}{}{}{}{}{}", m, FULL, m.txt, m, HINT, h)?;
    }
    term.flush()?;

    // Signal OK to swap
    sock.send_to(&[0], &path)?;

    let mut input = String::with_capacity(2); 
    loop {
        input.push(term.next()?);
        hints.retain(|(h, _)| h.starts_with(&input));
        if hints.len() <= 1 { break }
        hints.iter().try_for_each(|(_, m)| write!(&mut term, "{}{}{}", m, PICK, input))?;
        term.flush()?;
    }

    if let Some((_, m)) = hints.into_iter().next() {
        ClipboardContext::new()?.set_contents(m.txt.into())?;
    }

    Ok(drop(bomb))
}
