use std::{
    io::{self, Write},
    path::Path,
};

use crate::{
    commands::{command, command_output, command_silent},
    git_commands::helper::{git_messages, repo_has_commits, smart_stage},
    stage,
};

pub fn stage(files: Vec<String>) {
    let mut args: Vec<&str> = vec!["git", "add"];
    args.extend(files.iter().map(String::as_str));
    command(&args);
}

pub fn unstage(location: &str) {
    if repo_has_commits() {
        command(&["git", "restore", "--staged", location]);
    } else {
        command_silent(&["git", "rm", "-r", "--cached", location]);
    }
}

pub fn pull(force: bool, skip: bool) {
    if !force {
        command(&["git", "pull"]);
    } else {
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

        command(&["git", "fetch", "origin"]);
        command(&["git", "reset", "--hard", "@{u}"]);
    }
}

pub fn add_and_commit(message: Vec<String>, force: bool, all: bool) -> Result<(), String> {
    let status = stage::get_simple_status();
    smart_stage(&status, all)?;

    if message.is_empty() && force {
        let count = status.staged.len();
        let msg = format!(
            "Changed {} file{}",
            count,
            if count == 1 { "" } else { "s" }
        );
        let list_str = format!("Files changed: {}", status.staged.join(", "));
        command(&["git", "commit", "-m", &msg, "-m", &list_str]);
    } else {
        let mut args = vec!["git", "commit"];
        let mut messages = git_messages(&message);
        args.append(&mut messages);
        command(&args);
    }

    Ok(())
}

pub fn push(message: Vec<String>, force: bool, all: bool) -> Result<(), String> {
    add_and_commit(message, force, all).unwrap_or_else(|e| eprintln!("{e}"));
    command(if force {
        &["git", "push", "--force"]
    } else {
        &["git", "push"]
    });
    Ok(())
}

pub fn amend_commit(message: Vec<String>, all: bool) -> Result<(), String> {
    let _ = smart_stage(&stage::get_simple_status(), all);

    if message.len() == 0 {
        command(&["git", "commit", "--amend", "--no-edit"]);
    } else {
        let mut args = vec!["git", "commit", "--amend"];
        let mut messages = git_messages(&message);
        args.append(&mut messages);
        command(&args);
    }
    Ok(())
}

pub fn amend_push(message: Vec<String>, force: bool, all: bool) -> Result<(), String> {
    amend_commit(message, all)?;
    command(&[
        "git",
        "push",
        if force {
            "--force"
        } else {
            "--force-with-lease"
        },
    ]);
    Ok(())
}

pub fn undo_commit(hard: bool, commit: String) -> Result<(), String> {
    if repo_has_commits() {
        if hard {
            command(&["git", "reset", "--hard", &commit]);
        } else {
            command(&["git", "reset", &commit]);
        }
        Ok(())
    } else {
        Err(String::from("This repo has no commits."))
    }
}

pub fn undo_push(hard: bool, commit: String) -> Result<(), String> {
    undo_commit(hard, commit)?;
    command(&["git", "push", "--force-with-lease"]);
    Ok(())
}

pub fn revert_stage(files: Vec<String>) {
    let mut args: Vec<&str> = vec!["git", "restore", "--staged"];
    if files.len() > 0 {
        args.extend(files.iter().map(String::as_str));
    } else {
        args.push(".");
    }
    command(&args);
}

pub fn init(messages: Vec<String>, branch: String) -> Result<(), String> {
    if Path::new(".git").is_dir() {
        Err(String::from("This directory has already been initialized"))
    } else {
        command(&["git", "init"]);
        command(&["git", "branch", "-M", &branch]);
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
    command(&["git", "remote", "add", "origin", remote]);
}

pub fn push_to_origin() {
    let branch = command_output(&["git", "branch", "--show-current"]);
    command(&["git", "push", "-u", "origin", &branch.trim()]);
}
