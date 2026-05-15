use std::io::{self, Write};

use crate::commands::{run_command, run_command_output};

fn get_staged() -> Vec<String> {
    let changed = run_command_output(&["git", "diff", "--name-only", "--staged"]);
    let changed = changed.trim();
    changed
        .trim()
        .split('\n')
        .filter_map(|s| {
            if s.is_empty() {
                None
            } else {
                Some(s.to_string())
            }
        })
        .collect()
}

pub fn stage(location: &str) {
    run_command(&["git", "add", location]);
}

pub fn unstage(location: &str) {
    run_command(&["git", "restore", "--staged", location]);
}

fn git_messages(messages: &[String]) -> Vec<&str> {
    let mut res: Vec<&str> = Vec::new();

    for message in messages {
        res.push("-m");
        res.push(message);
    }

    res
}

pub fn add_and_commit(message: Vec<String>, force: bool, all: bool) {
    if get_staged().len() == 0 || all {
        run_command(&["git", "add", "."]);
    }

    if message.len() == 0 && force {
        let files = get_staged();
        let count = files.len();
        let msg = format!(
            "Changed {} file{}",
            count,
            if count == 1 { "" } else { "s" }
        );
        let list_str = format!("Files changed: {}", files.join(", "));
        run_command(&["git", "commit", "-m", &msg, "-m", &list_str]);
    } else {
        let mut args = vec!["git", "commit"];
        let mut messages = git_messages(&message);
        args.append(&mut messages);
        run_command(&args);
    }
}

pub fn amend_commit(message: Vec<String>) {
    run_command(&["git", "add", "."]);

    if message.len() == 0 {
        run_command(&["git", "commit", "--amend", "--no-edit"]);
    } else {
        let mut args = vec!["git", "commit"];
        let mut messages = git_messages(&message);
        args.append(&mut messages);
        run_command(&args);
    }
}

pub fn push(message: Vec<String>, force: bool, all: bool) {
    add_and_commit(message, force, all);
    run_command(if force {
        &["git", "push", "--force"]
    } else {
        &["git", "push"]
    });
}

pub fn amend_push(message: Vec<String>, force: bool) {
    amend_commit(message);
    run_command(&[
        "git",
        "push",
        if force {
            "--force"
        } else {
            "--force-with-lease"
        },
    ]);
}

pub fn pull() {
    run_command(&["git", "pull"]);
}

pub fn force_pull(skip: bool) {
    if !skip {
        let mut input = String::new();
        print!("This will wipe uncommited changes. Are you sure? [y/N] ");
        io::stdout().flush().expect("Could not print");
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading message");

        let response = input.trim().to_lowercase();
        let response = if response.is_empty() {
            String::from("n")
        } else {
            response
        };
        match response.as_str() {
            "n" => std::process::exit(0),
            "y" => (),
            _ => {
                println!("That is not a command");
                std::process::exit(1);
            }
        }
    }

    run_command(&["git", "fetch", "origin"]);
    run_command(&["git", "reset", "--hard", "@{u}"]);
}

pub fn undo_commit(hard: bool, commit: String) {
    if hard {
        run_command(&["git", "reset", "--hard", &commit]);
    } else {
        run_command(&["git", "reset", &commit]);
    }
}

pub fn undo_push(force: bool, hard: bool, commit: String) {
    undo_commit(hard, commit);
    run_command(&[
        "git",
        "push",
        if force {
            "--force"
        } else {
            "--force-with-lease"
        },
    ]);
}

pub fn revert_stage(files: Vec<String>) {
    let mut args: Vec<&str> = vec!["git", "restore", "--staged"];
    if files.len() > 0 {
        args.extend(files.iter().map(String::as_str));
    } else {
        args.push(".");
    }
    run_command(&args);
}

pub fn stage_files(files: Vec<String>) {
    let mut args: Vec<&str> = vec!["git", "add"];
    args.extend(files.iter().map(String::as_str));
    run_command(&args);
}

pub fn init(message: Vec<String>) {
    run_command(&["git", "init"]);
    add_and_commit(
        if message.len() == 0 {
            vec!["initial commit".to_owned()]
        } else {
            message
        },
        false,
        true,
    );
}

pub fn setup_remote(remote: &str) {
    run_command(&["git", "branch", "-M", "main"]);
    run_command(&["git", "remote", "add", "origin", remote]);
}

pub fn push_to_main() {
    run_command(&["git", "push", "-u", "origin", "main"]);
}
