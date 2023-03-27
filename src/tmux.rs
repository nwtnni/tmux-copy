use std::env;
use std::io;
use std::process;

#[derive(Clone, Debug)]
pub struct Pane {
    id: String,
    height: i32,
    position: i32,
}

/// Get the ID of the active `tmux` pane.
pub fn active() -> Result<Pane, io::Error> {
    let stdout = command!(
        "tmux",
        "list-panes",
        "-F",
        r"#{?#{&&:#{pane_active},#{window_active}},#{pane_id}@@#{pane_height}@@#{scroll_position},}"
    )
    .output()
    .map(stdout)?;

    let mut iter = stdout.trim().split("@@");
    let id = iter
        .next()
        .map(str::trim)
        .map(String::from)
        .expect("Missing pane ID");
    let height = iter
        .next()
        .expect("Missing pane height")
        .trim()
        .parse::<i32>()
        .expect("Invalid pane height");
    let position = iter
        .next()
        .expect("Missing scroll position")
        .trim()
        .parse::<i32>()
        .unwrap_or(0);

    Ok(Pane {
        id,
        height,
        position,
    })
}

/// Display `message` in the status line.
pub fn display(message: &str) -> Result<(), io::Error> {
    command!("tmux", "display-message", message)
        .spawn()?
        .wait()
        .map(drop)
}

/// Get the contents of `pane`.
pub fn capture(pane: &Pane, escapes: bool) -> Result<String, io::Error> {
    // https://github.com/fcsonline/tmux-thumbs/blob/ae91d5f7c0d989933e86409833c46a1eca521b6a/src/swapper.rs#L203-L208
    command!(
        "tmux",
        "capture-pane",
        if escapes { "-eJpt" } else { "-Jpt" },
        &pane.id,
        "-S",
        format!("{}", -pane.position),
        "-E",
        format!("{}", pane.height - pane.position - 1),
    )
    .output()
    .map(|mut out| {
        // Remove trailing newline
        out.stdout.pop();
        out
    })
    .map(stdout)
}

/// Send `text` to `pane` as if typed.
pub fn send(pane: &Pane, text: &str) -> Result<(), io::Error> {
    command!("tmux", "send-keys", "-t", &pane.id, text)
        .spawn()?
        .wait()
        .map(drop)
}

/// Swap the current pane with another.
pub fn swap(to: &Pane) -> Result<(), io::Error> {
    let from = env::var_os("TMUX_PANE").expect("Must be run in tmux");
    command!("tmux", "swap-pane", "-s", from, "-t", &to.id)
        .spawn()?
        .wait()
        .map(drop)
}

/// Get `stdout` of a process, assuming that it's UTF-8.
fn stdout(output: process::Output) -> String {
    String::from_utf8(output.stdout).expect("Invalid UTF-8")
}
