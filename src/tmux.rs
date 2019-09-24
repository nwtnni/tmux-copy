use std::error;
use std::process;

pub fn active() -> Result<String, Box<dyn error::Error>> {
    command!("tmux", "list-panes", "-F", "#{?pane_active,#{pane_id},}")
        .output()
        .map(|out| out.stdout)
        .map(String::from_utf8)?
        .map_err(From::from)
}

pub fn capture(pane: &str) -> Result<String, Box<dyn error::Error>> {
    command!("tmux", "capture-pane", "-Jpt", pane.trim())
        .output()
        .map(|out| out.stdout)
        .map(String::from_utf8)?
        .map_err(From::from)
}
