use crate::commands::{run_command, run_command_output};

/// Opens the url in the default browser
pub fn open(url: &str) {
    match std::env::consts::OS {
        "windows" => run_command(&["cmd", "/c", "start", url]),
        "macos" => run_command(&["open", url]),
        _ => run_command(&["xdg-open", url]),
    }
}

/// Get url of the git remote
pub fn get_project_url() -> String {
    let remote = run_command_output(&["git", "remote", "get-url", "origin"]);
    let remote = remote.trim();
    remote.replace(".git", "")
}

/// Get url of the git commit
pub fn get_commit_url(commit: &str) -> String {
    let remote = get_project_url();
    let hash = run_command_output(&["git", "rev-parse", commit]);
    let hash = hash.trim();
    format!("{remote}/commit/{hash}")
}
