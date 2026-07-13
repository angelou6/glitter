use crate::{commands, stage};

pub fn repo_has_commits() -> bool {
    commands::command_silent(&["git", "rev-parse", "--verify", "HEAD"]).success()
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
