use ::std::process::Command;
use std::{
    io::ErrorKind,
    process::{ExitStatus, Stdio},
};

/// Run command and output to stdout
pub fn command(args: &[&str]) -> ExitStatus {
    Command::new(args[0])
        .args(&args[1..])
        .status()
        .unwrap_or_else(|e| {
            eprintln!("{e}");
            std::process::exit(1);
        })
}

pub fn command_silent(args: &[&str]) -> ExitStatus {
    Command::new(args[0])
        .args(&args[1..])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .unwrap_or_else(|e| {
            eprintln!("{e}");
            std::process::exit(1);
        })
}

/// Run command return its output
pub fn command_output(args: &[&str]) -> String {
    let out = Command::new(args[0])
        .args(&args[1..])
        .output()
        .unwrap_or_else(|e| {
            eprintln!("{e}");
            std::process::exit(1);
        });
    String::from_utf8_lossy(&out.stdout).to_string()
}

/// Spawn a command
pub fn spawn_command(args: &[&str]) {
    Command::new(args[0])
        .args(&args[1..])
        .spawn()
        .unwrap_or_else(|e| {
            eprintln!("{e}");
            std::process::exit(1);
        });
}

pub fn command_exists(cmd: &str) -> bool {
    match Command::new(cmd).spawn() {
        Ok(mut child) => {
            let _ = child.kill();
            true
        }
        Err(e) => e.kind() != ErrorKind::NotFound,
    }
}
