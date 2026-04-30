use std::{
    io::{self, Write},
    process::Command,
};

/// Run a command print its output
pub fn run_command(args: &[&str]) {
    Command::new(args[0])
        .args(&args[1..])
        .status()
        .unwrap_or_else(|e| {
            eprintln!("{e}");
            std::process::exit(1);
        });
}

/// Run a command and get its output
pub fn run_command_output(args: &[&str]) -> String {
    let out = Command::new(args[0])
        .args(&args[1..])
        .output()
        .unwrap_or_else(|e| {
            eprintln!("{e}");
            std::process::exit(1);
        });
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

fn get_staged() -> Vec<String> {
    let changed = run_command_output(&["git", "diff", "--name-only", "--staged"]);
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

pub fn add_and_commit(message: Vec<String>, force: bool, all: bool) {
    if get_staged().len() == 0 || all {
        run_command(&["git", "add", "."]);
    }

    match message.len() {
        0 => {
            if force {
                let files = get_staged();
                let count = files.len();
                let msg = format!(
                    "Changed {} file{}",
                    count,
                    if count == 1 { "" } else { "s" }
                );
                let list_str = format!("Files changed: {}", files.join(", "));
                run_command(&["git", "commit", "-m", &msg, "-m", &list_str]);
            }
        }
        1 => run_command(&["git", "commit", "-m", &message[0]]),
        2 => run_command(&["git", "commit", "-m", &message[0], "-m", &message[1]]),
        _ => unreachable!(),
    }
}

pub fn amend_commit(message: Vec<String>) {
    run_command(&["git", "add", "."]);

    match message.len() {
        0 => run_command(&["git", "commit", "--amend", "--no-edit"]),
        1 => run_command(&["git", "commit", "--amend", "-m", &message[0]]),
        2 => run_command(&[
            "git",
            "commit",
            "--amend",
            "-m",
            &message[0],
            "-m",
            &message[1],
        ]),
        _ => unreachable!(),
    }
}

pub fn push(message: Vec<String>, force: bool, all: bool) {
    add_and_commit(message, force, all);

    if force {
        run_command(&["git", "push", "--force"]);
    } else {
        run_command(&["git", "push"]);
    }
}

pub fn amend_push(message: Vec<String>, force: bool) {
    amend_commit(message);
    let force = if force {
        "--force"
    } else {
        "--force-with-lease"
    };
    run_command(&["git", "push", force]);
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

pub fn undo_commit(hard: bool) {
    if hard {
        run_command(&["git", "reset", "--hard", "HEAD~1"]);
    } else {
        run_command(&["git", "reset", "HEAD~1"]);
    }
}

pub fn undo_push(force: bool, hard: bool) {
    undo_commit(hard);
    let force = if force {
        "--force"
    } else {
        "--force-with-lease"
    };
    run_command(&["git", "push", force]);
}
