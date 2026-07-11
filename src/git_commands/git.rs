use std::io::{self, Write};

use crossterm::terminal::is_raw_mode_enabled;

use crate::{
    commands::{command, command_output, command_silent},
    git_commands::utils,
};

pub fn stage(files: Vec<String>) {
    let mut args = vec!["git", "add"];
    args.extend(files.iter().map(String::as_str));
    command(&args);
}

pub fn unstage(files: Vec<String>) -> std::io::Result<()> {
    let mut args: Vec<&str> = vec!["git", "restore", "--staged"];

    if !utils::repo_has_commits() {
        args = vec!["git", "rm", "-r", "--cached"]
    }

    if files.len() > 0 {
        args.extend(files.iter().map(String::as_str));
    } else {
        args.push(".");
    }

    if is_raw_mode_enabled()? {
        command_silent(&args);
    } else {
        command(&args);
    }

    Ok(())
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
                "n".into()
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

pub fn add_and_commit(messages: Vec<String>, force: bool, all: bool) -> Result<(), String> {
    if utils::get_staged().is_empty() || all {
        stage(vec![".".into()]);
    }

    let mut args = vec!["git", "commit"];
    let staged = utils::get_staged();

    if messages.is_empty() && force {
        let msgs = vec![
            format!(
                "Changed {} file{}",
                staged.len(),
                if staged.len() == 1 { "" } else { "s" }
            ),
            format!("Files changed: {}", staged.join(", ")),
        ];
        args.append(&mut utils::git_messages(&msgs));
        command(&args);
    } else {
        args.append(&mut utils::git_messages(&messages));
        command(&args);
    }

    Ok(())
}

pub fn push(messages: Vec<String>, force: bool, all: bool) -> Result<(), String> {
    add_and_commit(messages, force, all)?;
    command(if force {
        &["git", "push", "--force"]
    } else {
        &["git", "push"]
    });
    Ok(())
}

pub fn amend_commit(message: Vec<String>, all: bool) -> Result<(), String> {
    let _ = utils::smart_stage(&utils::get_simple_status(), all);

    if message.len() == 0 {
        command(&["git", "commit", "--amend", "--no-edit"]);
    } else {
        let mut args = vec!["git", "commit", "--amend"];
        let mut messages = utils::git_messages(&message);
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
    if utils::repo_has_commits() {
        if hard {
            command(&["git", "reset", "--hard", &commit]);
        } else {
            command(&["git", "reset", &commit]);
        }
        Ok(())
    } else {
        Err("This repo has no commits.".into())
    }
}

pub fn undo_push(hard: bool, commit: String) -> Result<(), String> {
    undo_commit(hard, commit)?;
    command(&["git", "push", "--force-with-lease"]);
    Ok(())
}

pub fn init(messages: Vec<String>, branch: String) -> Result<(), String> {
    command(&["git", "init"]);
    command(&["git", "branch", "-M", &branch]);
    add_and_commit(
        if messages.is_empty() {
            vec!["Initial commit".into()]
        } else {
            messages
        },
        false, // Don't force
        true,  // Add all files
    )?;
    Ok(())
}

pub fn setup_origin(remote: &str) {
    command(&["git", "remote", "add", "origin", remote]);
}

pub fn push_to_origin() {
    let branch = command_output(&["git", "branch", "--show-current"]);
    command(&["git", "push", "-u", "origin", &branch.trim()]);
}
