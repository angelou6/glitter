use std::io;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType, disable_raw_mode},
};

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

pub fn draw(label: &str, default: &str, allow_spaces: bool) -> io::Result<String> {
    let mut stdout = io::stdout();
    let mut input = String::new();
    let mut pointer: usize = 0;

    execute!(
        stdout,
        terminal::Clear(ClearType::CurrentLine),
        Print(label),
        SetForegroundColor(Color::Grey),
        Print(if default.is_empty() {
            " (optional)".into()
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
                        input.insert(
                            pointer,
                            if !allow_spaces && c.is_whitespace() {
                                '-'
                            } else {
                                c
                            },
                        );
                        pointer += 1;
                        print_user_input(&mut stdout, &input, offset)?;
                    }
                }
            }
        }
    }
}
