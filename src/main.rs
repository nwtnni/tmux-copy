use std::error;

mod ansi;
mod find;
mod term;
mod tmux;

fn main() -> Result<(), Box<dyn error::Error>> {
    let pane = tmux::active()?;
    let text = tmux::capture(&pane)?;
    println!("{}", text);
    Ok(())
}
