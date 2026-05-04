use std::io::{self, Write};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    queue,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal,
};

use crate::commands::run_command;

fn is_cancel(key: &event::KeyEvent) -> bool {
    key.code == KeyCode::Esc
        || (key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL))
}

fn read_text(prompt: &str, required: bool) -> Option<String> {
    let mut out = io::stdout();
    let mut value = String::new();

    loop {
        queue!(
            out,
            cursor::MoveToColumn(0),
            terminal::Clear(terminal::ClearType::CurrentLine),
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Cyan),
            Print(prompt),
            ResetColor,
            SetAttribute(Attribute::Reset),
            SetForegroundColor(Color::DarkGrey),
            Print(": "),
            ResetColor,
            Print(&value),
        )
        .ok()?;
        out.flush().ok()?;

        if let Ok(Event::Key(key)) = event::read() {
            if is_cancel(&key) {
                queue!(out, Print("\n\r")).ok()?;
                out.flush().ok()?;
                break None;
            }
            match key.code {
                KeyCode::Enter => {
                    if !required || !value.is_empty() {
                        queue!(out, Print("\n\r")).ok()?;
                        out.flush().ok()?;
                        break Some(value);
                    }
                }
                KeyCode::Backspace => {
                    value.pop();
                }
                KeyCode::Char(c) => value.push(c),
                _ => {}
            }
        }
    }
}

fn read_visibility() -> Option<bool> {
    let mut out = io::stdout();
    let mut selected = 0usize;
    let mut first = true;

    loop {
        if !first {
            queue!(out, cursor::MoveUp(2), cursor::MoveToColumn(0)).ok()?;
        }
        first = false;

        for (i, (label, color)) in [("public", Color::Green), ("private", Color::Red)]
            .iter()
            .enumerate()
        {
            let is_selected = selected == i;
            queue!(out, terminal::Clear(terminal::ClearType::CurrentLine)).ok()?;
            if is_selected {
                queue!(
                    out,
                    SetForegroundColor(Color::Cyan),
                    Print("> "),
                    ResetColor,
                    SetAttribute(Attribute::Bold),
                    SetForegroundColor(*color),
                    Print(label),
                    SetAttribute(Attribute::Reset),
                    ResetColor,
                )
                .ok()?;
            } else {
                queue!(
                    out,
                    SetForegroundColor(Color::DarkGrey),
                    Print(format!("  {}", label)),
                    ResetColor,
                )
                .ok()?;
            }
            queue!(out, Print("\n\r")).ok()?;
        }
        out.flush().ok()?;

        if let Ok(Event::Key(key)) = event::read() {
            if is_cancel(&key) {
                break None;
            }
            match key.code {
                KeyCode::Up | KeyCode::Char('k') => {
                    if selected > 0 {
                        selected -= 1;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if selected < 1 {
                        selected += 1;
                    }
                }
                KeyCode::Enter => break Some(selected == 1),
                _ => {}
            }
        }
    }
}

pub fn draw() -> Option<(String, String, bool)> {
    terminal::enable_raw_mode().ok()?;
    let result = draw_inner();
    let _ = terminal::disable_raw_mode();
    result
}

fn draw_inner() -> Option<(String, String, bool)> {
    let mut out = io::stdout();

    let name = read_text("name", true)?;
    let description = read_text("description (default: no description)", false)?;

    queue!(
        out,
        SetAttribute(Attribute::Bold),
        SetForegroundColor(Color::Cyan),
        Print("visibility"),
        ResetColor,
        SetAttribute(Attribute::Reset),
        SetForegroundColor(Color::DarkGrey),
        Print(":"),
        ResetColor,
        Print("\n\r"),
    )
    .ok()?;
    out.flush().ok()?;

    let private = read_visibility()?;

    Some((name, description, private))
}

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
