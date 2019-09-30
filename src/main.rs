use std::collections::HashMap;
use std::error;
use std::env;
use std::io;
use std::io::Write;

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

fn main() -> Result<(), Box<dyn error::Error>> {

    let pane = env::args().nth(1).expect("Expected active pane");
    let text = tmux::capture(&pane)?;
    let trum = text.trim_end();
    let bomb = Bomb(&pane);

    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut term = term::Term::new(&mut stdin, &mut stdout)?;

    let matches = find::matches(&trum);

    write!(&mut term, "{}{}", trum, ansi::RED)?;

    let mut hints = hint::hints(matches.len())
        .zip(&matches)
        .inspect(|(h, m)| {
            write!(&mut term, "{}{}", m, h).unwrap()
        })
        .collect::<HashMap<_, _>>();

    term.flush()?;

    let mut input = String::with_capacity(2); 

    loop {
        let next = term.next()?;

        // Check for ESC key
        if next == '\x1B' { return Ok(()) }

        input.push(next);
        hints.retain(|hint, _| hint.starts_with(&input));

        // Check for match
        if hints.len() <= 1 { break }

        // Write out matching characters
        write!(&mut term, "{}", ansi::GREEN)?;
        for (_, m) in &hints {
            write!(&mut term, "{}{}", m, input)?;
        }
        term.flush()?;
    }

    if hints.is_empty() { return Ok(()) }

    let (_, m) = hints.into_iter().next().unwrap();
    let mut context: ClipboardContext = ClipboardProvider::new()?;
    context.set_contents(m.txt.to_owned())?;

    drop(bomb);
    Ok(())
}
