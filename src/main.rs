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

/// Destructor swaps back to original `tmux` pane.
struct Bomb<'pane>(&'pane str);

impl<'pane> Drop for Bomb<'pane> {
    fn drop(&mut self) {
        tmux::swap(&self.0).ok();
    }
}

/// Color of matching text
const FULL: ansi::Color = ansi::Color(6);

/// Color of unselected hint
const HINT: ansi::Color = ansi::Color(10);

/// Color of selected hint
const PICK: ansi::Color = ansi::Color(11);

fn main() -> Result<(), Box<dyn error::Error>> {

    // Retrieve active pane ID and socket path from arguments
    let pane = env::args().nth(1).expect("Missing active pane");

    // Set up I/O
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut term = term::Term::new(&mut stdin, &mut stdout)?;

    // Search for matches
    let capture = tmux::capture(&pane)?;
    let matches = find::matches(&capture);

    // Short-circuit without swapping if there are no matches
    if matches.is_empty() {
        tmux::display("No matches found.")?;
        return Ok(())
    }

    let mut hints = hint::hints(matches.len())
        .zip(&matches)
        .collect::<Vec<_>>();

    // Write out original text, matches, and hints
    tmux::render(&pane, &mut term)?;
    for (h, m) in &hints {
        write!(&mut term, "{}{}{}{}{}{}", m, FULL, m.txt, m, HINT, h)?;
    }
    term.flush()?;

    // Swap with active pane
    tmux::swap(&pane)?;
    
    // Ensure that we swap back
    let bomb = Bomb(&pane);

    // Blocking reads for user input
    let mut input = String::with_capacity(2); 
    loop {
        input.push(term.next()?);
        hints.retain(|(h, _)| h.starts_with(&input));
        if hints.len() <= 1 { break }
        hints.iter().try_for_each(|(_, m)| write!(&mut term, "{}{}{}", m, PICK, input))?;
        term.flush()?;
    }

    // Copy selected text
    if let Some((_, m)) = hints.into_iter().next() {
        ClipboardContext::new()?.set_contents(m.txt.into())?;
    }

    Ok(drop(bomb))
}
