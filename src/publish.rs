use crate::commands::run_command;
use crate::tui::select::Selector;
use crate::tui::{input, select};
use crossterm::execute;
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::env;
use std::io;

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

fn draw_visibility_selection() -> io::Result<bool> {
    let mut selector: Selector<String> = select::new(vec!["Private".into(), "Public".into()]);

    loop {
        match selector.draw()? {
            Some(select::SelectVal::KeyboardInterrupt) | Some(select::SelectVal::Quit) => {
                return Err(io::Error::new(io::ErrorKind::Other, "Keyboard Interrupt"));
            }
            Some(select::SelectVal::Select) => {
                return Ok(selector.options[selector.pointer] == "Private");
            }
            _ => {}
        }
    }
}

pub fn draw() -> io::Result<(String, String, bool)> {
    let cwd = env::current_dir().unwrap();
    let cwd = cwd.file_name().unwrap().to_string_lossy();

    enable_raw_mode()?;
    let name = input::draw("Name", &cwd)?;
    let desc = input::draw("Description", "")?;

    execute!(io::stdout(), Print("Visibility:"), Print("\r\n"))?;
    let is_private = draw_visibility_selection()?;
    disable_raw_mode()?;

    Ok((name, desc, is_private))
}
