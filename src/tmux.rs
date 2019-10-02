use std::io;
use std::path;
use std::process;

/// Get the ID of the active `tmux` pane.
pub fn active() -> Result<String, io::Error> {
    command!("tmux", "list-panes", "-F", "#{?#{&&:#{pane_active},#{window_active}},#{pane_id},}")
        .output()
        .map(stdout)
}

/// Get the plain-text contents of `pane`.
pub fn capture(pane: &str) -> Result<String, io::Error> {
    command!("tmux", "capture-pane", "-pt", pane.trim())
        .output()
        .map(|mut out| {
            // Remove trailing newline
            out.stdout.pop();
            out
        })
        .map(stdout)
}

/// Write the full content (including ANSI escape sequences) of `pane` to `to`.
pub fn render<W: io::Write>(pane: &str, mut to: W) -> Result<(), io::Error> {
    command!("tmux", "capture-pane", "-ept", pane.trim())
        .output()
        .and_then(|mut out| {
            // Remove trailing newline
            out.stdout.pop();
            to.write_all(&out.stdout)
        })
}

/// Spawn an instance of `main` and get its pane ID.
pub fn spawn<P: AsRef<path::Path>>(pane: &str, addr: P, uuid: &str) -> Result<String, io::Error> {
    let main = format!(
        "target/release/main {} {}",
        pane.trim(),
        addr.as_ref().display()
    );

    command!("tmux", "new-window", "-dn", uuid, main)
        .spawn()?
        .wait()
        .map(drop)?;

    let panes = command!("tmux", "list-panes", "-aF", r"#W #D")
        .output()
        .map(stdout)?;

    let pane = panes.split('\n')
        .find(|id| id.starts_with(uuid))
        .expect("Pane was never created");

    Ok(
        pane.split(' ')
            .nth(1)
            .expect("list-panes formatting string is incorrect")
            .trim()
            .to_owned()
    )
}

/// Swap two panes.
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

/// Get `stdout` of a process, assuming that it's UTF-8.
fn stdout(output: process::Output) -> String {
    String::from_utf8(output.stdout).expect("Invalid UTF-8")
}
