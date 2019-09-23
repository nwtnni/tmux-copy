use std::error;

mod tmux;

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("{}", tmux::active()?);
    Ok(())
}
