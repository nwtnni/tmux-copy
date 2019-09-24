use std::collections::HashMap;
use std::error;
use std::io;

#[macro_use]
mod util;
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

    write!(&mut term, "{}", text)?;

    let matches = find::matches(&text);
    let mut map = HashMap::with_capacity(matches.len());

    write!(&mut term, "{}", ansi::RED)?;

    for (r#match, hint) in matches.iter().zip(hint::hints(matches.len())) {
        map.insert(hint, r#match);
        let go = ansi::Go(r#match.col as u16, r#match.row as u16);
        write!(&mut term, "{}{}", go, hint)?;
    }

    term.flush()?;

    let mut input = String::with_capacity(2); 

    while map.len() > 1 {

        let next = term.next()?;

        // Check for ESC key
        if next == '\x1B' { return Ok(()) }

        input.push(next);
        map.retain(|hint, _| hint.starts_with(&input));

        write!(&mut term, "{}", ansi::GREEN)?;

        for (_, r#match) in map.iter() {
            let go = ansi::Go(r#match.col as u16, r#match.row as u16);
            write!(&mut term, "{}{}", go, input)?;
        }

        term.flush()?;
    }

    if map.len() == 1 {
        let r#match = map.into_iter()
            .next()
            .unwrap();
    }

    Ok(())
}
