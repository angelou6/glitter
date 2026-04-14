use std::{io::{self, Write}, process::Command};

fn run_command(command: &str) {
    let args: Vec<&str> = command.split(" ").collect();

    Command::new(args[0])
        .args(&args[1..])
        .status()
        .unwrap_or_else(|e| {
           eprintln!("{e}");
           std::process::exit(1);
        });
}

fn add_and_commit(message: &str, force: bool) {
    let message = if force { "I don't know" } else { message };

    run_command("git add .");
    run_command(&format!("git commit -m {message}"));
}

fn amend_commit(message: &str) {
    run_command("git add .");

    if !message.is_empty() {
        run_command("git commit --amend --no-edit");
    } else {
        run_command(&format!("git commit --amend -m {message}"));
    }
}

pub fn push(message: &str, blame: &str, force: bool) {
    add_and_commit(message, force);

    if !blame.is_empty() {
        run_command(&format!("git commit --amend --author {blame} --no-edit"));
    }

    if force {
        run_command("git push --force");
    } else {
        run_command("git push");
    }
}

pub fn force_pull(skip: bool) {
    if !skip {
        let mut input = String::new();
        print!("This will wipe all uncommited changes. Are you sure? [y/N] ");
        io::stdout().flush().expect("Could not print");
        io::stdin().read_line(&mut input).expect("Error reading message");

        let response = input.trim().to_lowercase();
        let response = if response.is_empty() { String::from("n") } else { response };
        match response.as_str() {
            "n" => std::process::exit(0),
            "y" => (),
            _ => {
                println!("That is not a command");
                std::process::exit(1);
            }
        }
    }

    run_command("git fetch --all");
    run_command("git reset --hard");
}


pub fn push_as_last(message: &str, force: bool) {
    amend_commit(message);
    let force = if force { "--force" } else { "--force-with-lease" };
    run_command(&format!("git push {force}"));
}
