use std::path::Path;

use crate::{
    commands::{command, command_silent},
    tui::stage,
};

pub fn git_messages(messages: &[String]) -> Vec<&str> {
    let mut res: Vec<&str> = Vec::new();

    for message in messages {
        res.push("-m");
        res.push(message);
    }

    res
}

pub fn smart_stage(status: &stage::Status, all: bool) -> Result<(), String> {
    if status.staged.is_empty() && status.unstaged.is_empty() {
        return Err("Nothing to commit".into());
    } else if status.staged.is_empty() || all {
        command(&["git", "add", "."]);
    }
    Ok(())
}

pub fn repo_has_commits() -> bool {
    command_silent(&["git", "rev-parse", "--verify", "HEAD"]).success()
}

pub fn is_repo() -> bool {
    Path::new(".git").is_dir()
}
