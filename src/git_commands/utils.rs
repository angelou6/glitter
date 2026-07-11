use crate::{
    commands,
    stage::{self, Status},
};

pub fn repo_has_commits() -> bool {
    commands::command_silent(&["git", "rev-parse", "--verify", "HEAD"]).success()
}

pub fn smart_stage(status: &stage::Status, all: bool) -> Result<(), String> {
    if status.staged.is_empty() && status.unstaged.is_empty() {
        return Err("Nothing to commit".into());
    } else if status.staged.is_empty() || all {
        commands::command_silent(&["git", "add", "."]);
    }
    Ok(())
}

pub fn git_messages(messages: &[String]) -> Vec<&str> {
    let mut res: Vec<&str> = Vec::new();

    for message in messages {
        res.push("-m");
        res.push(message);
    }

    res
}

pub fn get_staged() -> Vec<String> {
    stage::parse_status()
        .iter()
        .filter_map(|f| {
            if f.is_tracked {
                Some(f.path.clone())
            } else {
                None
            }
        })
        .collect()
}

pub fn get_simple_status() -> Status {
    let status = stage::parse_status();
    let (staged, unstaged): (Vec<_>, Vec<_>) = status.iter().partition(|f| f.is_tracked);

    Status {
        staged: staged.into_iter().map(|f| f.path.clone()).collect(),
        unstaged: unstaged.into_iter().map(|f| f.path.clone()).collect(),
    }
}
