use ::std::process::Command;

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
    String::from_utf8_lossy(&out.stdout).to_string()
}
