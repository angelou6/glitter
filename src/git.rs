use std::{
    io::{self, Write},
    path::Path,
};

use crate::{
    commands::{run_command, run_command_output},
    stage,
};

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

pub fn add_and_commit(message: Vec<String>, force: bool, all: bool) -> Result<(), String> {
    let status = stage::get_simple_status();

    if status.staged.is_empty() && status.unstaged.is_empty() {
        return Err(String::from("Nothing to commit"));
    } else if status.staged.is_empty() || all {
        run_command(&["git", "add", "."]);
    }

    if message.is_empty() && force {
        let count = status.staged.len();
        let msg = format!(
            "Changed {} file{}",
            count,
            if count == 1 { "" } else { "s" }
        );
        let list_str = format!("Files changed: {}", status.staged.join(", "));
        run_command(&["git", "commit", "-m", &msg, "-m", &list_str]);
    } else {
        let mut args = vec!["git", "commit"];
        let mut messages = git_messages(&message);
        args.append(&mut messages);
        run_command(&args);
    }

    Ok(())
}

pub fn amend_commit(message: Vec<String>) {
    run_command(&["git", "add", "."]);

    if message.len() == 0 {
        run_command(&["git", "commit", "--amend", "--no-edit"]);
    } else {
        let mut args = vec!["git", "commit", "--amend"];
        let mut messages = git_messages(&message);
        args.append(&mut messages);
        run_command(&args);
    }
}

pub fn push(message: Vec<String>, force: bool, all: bool) -> Result<(), String> {
    add_and_commit(message, force, all).unwrap_or_else(|e| eprintln!("{e}"));
    run_command(if force {
        &["git", "push", "--force"]
    } else {
        &["git", "push"]
    });
    Ok(())
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

pub fn undo_push(hard: bool, commit: String) {
    undo_commit(hard, commit);
    run_command(&["git", "push", "--force-with-lease"]);
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

pub fn init(messages: Vec<String>, branch: String) -> Result<(), String> {
    if Path::new(".git").is_dir() {
        Err(String::from("This directory has already been initialized"))
    } else {
        run_command(&["git", "init"]);
        run_command(&["git", "branch", "-M", &branch]);
        add_and_commit(
            if messages.is_empty() {
                vec!["initial commit".into()]
            } else {
                messages
            },
            false, // Don't force
            true,  // Stage all files
        )?;
        Ok(())
    }
}

pub fn setup_origin(remote: &str) {
    run_command(&["git", "remote", "add", "origin", remote]);
}

pub fn push_to_origin() {
    let branch = run_command_output(&["git", "branch", "--show-current"]);
    run_command(&["git", "push", "-u", "origin", &branch.trim()]);
}
