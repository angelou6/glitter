use ::std::process::Command;
use std::io::ErrorKind;

/// Run a command output to stdout
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
