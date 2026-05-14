use std::env;
use std::io::{self, Write};

use crate::commands::run_command;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use crossterm::terminal::{self, Clear, ClearType, disable_raw_mode, enable_raw_mode};
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

fn pop_until_space(input: &mut String, pointer: &mut usize) {
    let trimmed = input.trim_end();

    if let Some(index) = trimmed.rfind(' ') {
        input.truncate(index + 1);
        *pointer = index + 1;
    } else {
        input.clear();
        *pointer = 0;
    }
}

/// Only update the user input instead of redrawing the whole line
fn print_user_input(stdout: &mut io::Stdout, input: &str, offset: u16) -> io::Result<()> {
    execute!(
        stdout,
        cursor::MoveToColumn(offset),
        Clear(ClearType::UntilNewLine),
        Print(input)
    )?;
    Ok(())
}

fn draw_input(label: &str, default: &str) -> io::Result<String> {
    let mut stdout = io::stdout();
    let mut input = String::new();
    let mut pointer: usize = 0;

    execute!(
        stdout,
        terminal::Clear(ClearType::CurrentLine),
        Print(label),
        SetForegroundColor(Color::Grey),
        Print(if default.is_empty() {
            String::from(" (optional)")
        } else {
            format!(" (default: {default})")
        }),
        ResetColor,
        Print(": ")
    )?;
    let (offset, _) = cursor::position()?;

    loop {
        if let Event::Key(key) = event::read()? {
            match (key.code, key.modifiers) {
                (KeyCode::Char('c'), KeyModifiers::CONTROL)
                | (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                    disable_raw_mode()?;
                    return Err(io::Error::new(io::ErrorKind::Other, "Keyboard Interrupt"));
                }
                (KeyCode::Enter, _) => {
                    if input.is_empty() {
                        execute!(
                            stdout,
                            if !default.is_empty() {
                                Print(default)
                            } else {
                                Print("(empty)")
                            }
                        )?;
                    }
                    execute!(stdout, Print("\r\n"))?;

                    return Ok(if input.is_empty() { default } else { &input }.to_owned());
                }
                (KeyCode::Backspace, modifier) => {
                    if input.len() == 0 {
                        continue;
                    }

                    if modifier == KeyModifiers::ALT {
                        pop_until_space(&mut input, &mut pointer);
                    } else {
                        input.pop();
                        pointer -= 1;
                    }
                    print_user_input(&mut stdout, &input, offset)?;
                }
                (KeyCode::Char('e'), KeyModifiers::CONTROL) => {
                    execute!(stdout, cursor::MoveToColumn(offset + input.len() as u16))?;
                }
                (KeyCode::Char('a'), KeyModifiers::CONTROL) => {
                    execute!(stdout, cursor::MoveToColumn(offset))?;
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
                        print_user_input(&mut stdout, &input, offset)?;
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
            match (key.code, key.modifiers) {
                (KeyCode::Char('c'), KeyModifiers::CONTROL)
                | (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                    execute!(stdout, cursor::RestorePosition, cursor::Show)?;
                    disable_raw_mode()?;
                    return Err(io::Error::new(io::ErrorKind::Other, "Keyboard Interrupt"));
                }
                (KeyCode::Char('q'), _) | (KeyCode::Esc, _) => {
                    execute!(stdout, cursor::RestorePosition, cursor::Show)?;
                    disable_raw_mode()?;
                    return Err(io::Error::new(io::ErrorKind::Other, "Keyboard Interrupt"));
                }
                (KeyCode::Down, _) => {
                    pointer = if pointer + 1 < options.len() {
                        pointer + 1
                    } else {
                        0
                    }
                }
                (KeyCode::Up, _) => {
                    pointer = if pointer > 0 {
                        pointer - 1
                    } else {
                        options.len() - 1
                    }
                }
                (KeyCode::Enter, _) => {
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
    let desc = draw_input("Description", "")?;

    execute!(io::stdout(), Print("Visibility:"), Print("\r\n"))?;
    let is_private = draw_visibility_selection()?;

    disable_raw_mode()?;

    Ok((name, desc, is_private))
}
