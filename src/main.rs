use std::error;

mod tmux;

fn main() -> Result<(), Box<dyn error::Error>> {
    let pane = tmux::active()?;
    let text = tmux::capture(&pane)?;
    println!("{}", text);
    Ok(())
}
