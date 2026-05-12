use std::env;
use std::io::{self, Write};

use crate::commands::run_command;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use crossterm::terminal::{self, ClearType, disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, execute, queue};

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

fn print_label(stdout: &mut io::Stdout, label: &str, input: &str) -> io::Result<()> {
    execute!(
        stdout,
        cursor::MoveToColumn(0),
        terminal::Clear(ClearType::CurrentLine),
        Print(label),
        Print(input)
    )
}

fn pop_until_space(input: &mut String, pointer: &mut usize) {
    while !input.ends_with(' ') && input.len() > 0 {
        input.pop();
        *pointer -= 1;
    }
}

fn draw_input(label: &str, default: &str) -> io::Result<String> {
    let mut stdout = io::stdout();
    let custom_label = format!(
        "{label}{}: ",
        if default.is_empty() {
            String::new()
        } else {
            format!(", (default: {default})")
        }
    );

    let mut input = String::new();
    let mut pointer: usize = 0;

    print_label(&mut stdout, &custom_label, &input)?;

    loop {
        if let Event::Key(key) = event::read()? {
            match (key.code, key.modifiers) {
                (KeyCode::Char('c'), KeyModifiers::CONTROL)
                | (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                    disable_raw_mode()?;
                    return Err(io::Error::new(io::ErrorKind::Other, "Keyboard Interrupt"));
                }
                (KeyCode::Enter, _) => {
                    execute!(io::stdout(), Print("\r\n"))?;
                    return Ok(String::from(if input.is_empty() {
                        default
                    } else {
                        &input
                    }));
                }
                (KeyCode::Backspace, modifier) => {
                    if input.len() == 0 {
                        continue;
                    }

                    if modifier == KeyModifiers::ALT {
                        input.pop();
                        pop_until_space(&mut input, &mut pointer);
                    } else {
                        input.pop();
                    }
                    pointer -= 1;
                    print_label(&mut stdout, &custom_label, &input)?;
                }
                (KeyCode::Char('e'), KeyModifiers::CONTROL) => {
                    let (_, row) = cursor::position()?;
                    execute!(
                        stdout,
                        cursor::MoveTo((custom_label.len() + input.len()) as u16, row)
                    )?;
                }
                (KeyCode::Char('a'), KeyModifiers::CONTROL) => {
                    execute!(stdout, cursor::MoveToColumn(custom_label.len() as u16))?;
                }
                (KeyCode::Left, _) => {
                    if pointer - 1 > 0 {
                        pointer -= 1;
                        execute!(stdout, cursor::MoveLeft(1))?;
                    }
                }
                (KeyCode::Right, _) => {
                    if pointer < input.len() {
                        pointer += 1;
                        execute!(stdout, cursor::MoveRight(1))?;
                    }
                }
                other => {
                    if let Some(c) = other.0.as_char() {
                        input.insert(pointer, c);
                        pointer += 1;
                        print_label(&mut stdout, &custom_label, &input)?;
                    }
                }
            }
        }
    }
}

/// Returns true if Prvate is selected
fn draw_visibility_selection() -> io::Result<bool> {
    let mut stdout = io::stdout();
    let mut pointer: usize = 0;

    let options = ["Private", "Public"];

    execute!(stdout, cursor::Hide)?;

    loop {
        for (i, op) in options.iter().enumerate() {
            queue!(
                stdout,
                SetForegroundColor(Color::Blue),
                Print(if i == pointer { "> " } else { "  " }),
                ResetColor,
                Print(op),
                Print("\r\n"),
            )?;
        }
        queue!(
            stdout,
            cursor::SavePosition,
            cursor::MoveToPreviousLine(options.len() as u16)
        )?;
        stdout.flush()?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    execute!(stdout, cursor::RestorePosition, cursor::Show)?;
                    disable_raw_mode()?;
                    return Err(io::Error::new(io::ErrorKind::Other, "Keyboard Interrupt"));
                }
                KeyCode::Down => {
                    if pointer + 1 < options.len() {
                        pointer += 1;
                    }
                }
                KeyCode::Up => {
                    if pointer > 0 {
                        pointer -= 1;
                    }
                }
                KeyCode::Enter => {
                    execute!(stdout, cursor::RestorePosition, cursor::Show)?;
                    return Ok(pointer == 0);
                }
                _ => {}
            }
        }
    }
}

pub fn draw() -> io::Result<(String, String, bool)> {
    let cwd = env::current_dir().unwrap();
    let cwd = cwd.file_name().unwrap().to_string_lossy();

    enable_raw_mode()?;

    let name = draw_input("Name", &cwd)?;
    let desc = draw_input("Desc", "")?;

    let is_private = draw_visibility_selection()?;

    disable_raw_mode()?;

    Ok((name, desc, is_private))
}
