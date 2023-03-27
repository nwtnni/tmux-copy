use std::env;
use std::io;
use std::process;

/// Get the ID of the active `tmux` pane.
pub fn active() -> Result<String, io::Error> {
    command!(
        "tmux",
        "list-panes",
        "-F",
        "#{?#{&&:#{pane_active},#{window_active}},#{pane_id},}"
    )
    .output()
    .map(stdout)
}

/// Display `message` in the status line.
pub fn display(message: &str) -> Result<(), io::Error> {
    command!("tmux", "display-message", message)
        .spawn()?
        .wait()
        .map(drop)
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
    #[cfg(feature = "fade")]
    const FLAGS: &str = "-pt";

    #[cfg(not(feature = "fade"))]
    const FLAGS: &str = "-ept";

    command!("tmux", "capture-pane", FLAGS, pane.trim())
        .output()
        .and_then(|mut out| {
            // Remove trailing newline
            out.stdout.pop();
            to.write_all(&out.stdout)
        })
}

/// Send `text` to `pane` as if typed.
pub fn send(pane: &str, text: &str) -> Result<(), io::Error> {
    command!("tmux", "send-keys", "-t", pane.trim(), text)
        .spawn()?
        .wait()
        .map(drop)
}

/// Swap the current pane with another.
pub fn swap(to: &str) -> Result<(), io::Error> {
    let from = env::var_os("TMUX_PANE").expect("Must be run in tmux");
    command!("tmux", "swap-pane", "-s", from, "-t", to.trim())
        .spawn()?
        .wait()
        .map(drop)
}

/// Get `stdout` of a process, assuming that it's UTF-8.
fn stdout(output: process::Output) -> String {
    String::from_utf8(output.stdout).expect("Invalid UTF-8")
}
