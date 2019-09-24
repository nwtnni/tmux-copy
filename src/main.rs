use std::error;
use std::io;

mod ansi;
mod find;
mod hint;
mod term;
mod tmux;

use std::io::Write;

fn main() -> Result<(), Box<dyn error::Error>> {
    let pane = tmux::active()?;
    let text = tmux::capture(&pane)?;

    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut term = term::Term::new(&mut stdin, &mut stdout)?;

    let matches = find::matches(&text);
    let hints = hint::hints(matches.len());

    write!(&mut term, "{}", text)?;

    for (r#match, hint) in matches.iter().zip(hints) {
        let go = ansi::Go(r#match.col as u16, r#match.row as u16);
        write!(&mut term, "{}{}{}{}", go, ansi::COLOR, hint, ansi::RESET)?;
    }

    term.flush()?;

    std::thread::sleep_ms(5000);

    Ok(())
}
