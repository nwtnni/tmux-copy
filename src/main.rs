use std::error;
use std::io;
use std::io::Write;

use tmux_copy::ansi;
use tmux_copy::find;
use tmux_copy::hint;
use tmux_copy::term;
use tmux_copy::tmux;

/// Destructor swaps back to original `tmux` pane.
struct Bomb<'pane>(&'pane tmux::Pane);

impl<'pane> Drop for Bomb<'pane> {
    fn drop(&mut self) {
        tmux::swap(self.0).ok();
    }
}

/// Color of matching text
const FULL: ansi::Color = ansi::Color(6);

#[cfg(feature = "fade")]
/// Color of unmatched text
const FADE: ansi::Color = ansi::Color(8);

/// Color of unselected hint
const HINT: ansi::Color = ansi::Color(10);

/// Color of selected hint
const PICK: ansi::Color = ansi::Color(11);

fn main() -> Result<(), Box<dyn error::Error + Send + Sync>> {
    let mut clipboard = copypasta_ext::try_context().expect("Failed to initialize clipboard");

    // Set up I/O
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut term = term::Term::new(&mut stdin, &mut stdout)?;

    // Search for matches
    let pane = tmux::active()?;
    let capture = tmux::capture(&pane, false)?;
    let matches = find::matches(&capture);

    // Short-circuit without swapping if there are no matches
    if matches.is_empty() {
        tmux::display("No matches found.")?;
        return Ok(());
    }

    let mut candidates = hint::hints(matches.len()).zip(&matches).collect::<Vec<_>>();

    #[cfg(feature = "fade")]
    {
        write!(&mut term, "{}{}", FADE, capture)?;
    }
    #[cfg(not(feature = "fade"))]
    {
        write!(&mut term, "{}", tmux::capture(&pane, true)?)?;
    }

    for (hint, r#match) in &candidates {
        write!(
            &mut term,
            "{}{}{}{}{}{}",
            r#match, FULL, r#match.txt, r#match, HINT, hint,
        )?;
    }
    term.flush()?;

    // Swap with active pane
    tmux::swap(&pane)?;

    let bomb = Bomb(&pane);

    // Blocking reads for user input
    let mut input = String::with_capacity(2);
    loop {
        input.push(term.read()?);
        candidates.retain(|(hint, _)| {
            hint.chars()
                .zip(input.chars())
                .all(|(a, b)| a.eq_ignore_ascii_case(&b))
        });
        if candidates.len() <= 1 {
            break;
        }
        candidates
            .iter()
            .try_for_each(|(_, r#match)| write!(&mut term, "{}{}{}", r#match, PICK, input))?;
        term.flush()?;
    }

    match candidates.into_iter().next() {
        None => tmux::display("Selection cancelled.")?,
        Some((_, r#match)) if input.contains(char::is_uppercase) => tmux::send(&pane, r#match.txt)?,
        Some((_, r#match)) => {
            clipboard.set_contents(String::from(r#match.txt))?;
            tmux::display(&format!("Selected `{}`.", r#match.txt))?;
        }
    }

    drop(bomb);
    Ok(())
}
