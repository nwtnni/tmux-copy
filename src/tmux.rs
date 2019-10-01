use std::io;
use std::path;
use std::process;

const WINDOW_NAME: &str = "[tmux-copy]";

pub fn active() -> Result<String, io::Error> {
    command!("tmux", "list-panes", "-F", "#{?pane_active,#{pane_id},}")
        .output()
        .map(stdout)
}

pub fn capture(pane: &str) -> Result<String, io::Error> {
    command!("tmux", "capture-pane", "-pt", pane.trim())
        .output()
        .map(stdout)
}

pub fn render<W: io::Write>(pane: &str, mut to: W) -> Result<(), io::Error> {
    command!("tmux", "capture-pane", "-ept", pane.trim())
        .output()
        .and_then(|mut out| {
            // Remove trailing newline
            out.stdout.pop();
            to.write_all(&out.stdout)
        })
}

pub fn spawn<P: AsRef<path::Path>>(pane: &str, addr: P) -> Result<String, io::Error> {
    let main = format!(
        "target/release/main {} {}",
        pane.trim(),
        addr.as_ref().display()
    );

    command!("tmux", "new-window", "-dn", WINDOW_NAME, main)
        .spawn()?
        .wait()
        .map(drop)?;

    let panes = command!("tmux", "list-panes", "-aF", r"#W #D")
        .output()
        .map(stdout)?;

    let pane = panes.split('\n')
        .find(|id| id.starts_with(WINDOW_NAME))
        .expect("Pane was never created");

    Ok(
        pane.split(' ')
            .nth(1)
            .expect("Pane formatter is incorrect")
            .trim()
            .to_owned()
    )
}

pub fn swap(from: Option<&str>, to: &str) -> Result<(), io::Error> {
    let mut swap = command!("tmux", "swap-pane", "-d", "-t", to.trim());
    if let Some(from) = from {
        swap.arg("-s");
        swap.arg(from.trim());
    }
    swap.spawn()?
        .wait()
        .map(drop)
}

fn stdout(output: process::Output) -> String {
    String::from_utf8(output.stdout).expect("Invalid UTF-8")
}
