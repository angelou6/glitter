use crate::commands::{run_command, run_command_output};

pub fn open(url: &str) {
    match std::env::consts::OS {
        // I have no idea if it actually works on windows
        // but im also not going to test it
        "windows" => {
            run_command(&["cmd", "/c", "start", url]);
        }
        "macos" => {
            run_command(&["open", url]);
        }
        _ => {
            run_command(&["xdg-open", url]);
        }
    }
}

pub fn get_project_url() -> String {
    let remote = run_command_output(&["git", "remote", "get-url", "origin"]);
    return remote.replace(".git", "");
}

pub fn get_commit_url(commit: &str) -> String {
    let remote = get_project_url();
    let hash = run_command_output(&["git", "rev-parse", commit]);
    return format!("{remote}/commit/{hash}")
}
