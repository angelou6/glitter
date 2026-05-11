use std::io;

use crate::commands::run_command;

// Maybe at some point add forgejo/codeberg support if I ever move there
pub fn github(name: String, description: String, private: bool) {
    run_command(&[
        "gh",
        "repo",
        "create",
        &name,
        "--description",
        &description,
        "--source",
        ".",
        "--remote=origin",
        "--push",
        if private { "--private" } else { "--public" },
    ]);
}

pub fn draw() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    println!("You entered: {}", input.trim());
}
