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

/// Color of matching text
const FULL: ansi::Color = ansi::Color(6);

/// Color of unselected hint
const HINT: ansi::Color = ansi::Color(10);

/// Color of selected hint
const PICK: ansi::Color = ansi::Color(11);

fn main() -> Result<(), Box<dyn error::Error>> {

    let pane = env::args().nth(1).expect("Expected active pane");
    let bomb = Bomb(&pane);
    let text = tmux::capture_text(&pane)?;
    let show = tmux::capture_all(&pane)?;

    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut term = term::Term::new(&mut stdin, &mut stdout)?;

    let matches = find::matches(&text.trim_end());
    let mut hints = hint::hints(matches.len())
        .zip(&matches)
        .collect::<HashMap<_, _>>();

    // Write out original text, matches, and hints
    write!(&mut term, "{}", show.trim_end())?;
    hints.iter().try_for_each(|(h, m)| {
        write!(&mut term, "{}{}{}", m, FULL, m.txt)?;
        write!(&mut term, "{}{}{}", m, HINT, h)
    })?;
    term.flush()?;

    let mut input = String::with_capacity(2); 
    while hints.len() > 1 {
        input.push(term.next()?);
        hints.retain(|hint, _| hint.starts_with(&input));
        hints.iter().try_for_each(|(_, m)| write!(&mut term, "{}{}{}", m, PICK, input))?;
        term.flush()?;
    }

    if let Some((_, m)) = hints.into_iter().next() {
        ClipboardContext::new()?.set_contents(m.txt.into())?;
    }

    Ok(drop(bomb))
}
