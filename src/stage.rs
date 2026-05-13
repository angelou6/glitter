use std::io::{self, Write};

use crate::{commands, git};
use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, queue};

struct File {
    path: String,
    full_str: String,
    is_tracked: bool,
}

impl File {
    fn toggle_stage(&mut self) {
        if self.is_tracked {
            git::unstage(&self.path);
        } else {
            git::stage(&self.path);
        }
        self.is_tracked = !self.is_tracked;
    }
}

fn parse_porcelain() -> Vec<File> {
    let status = commands::run_command_output(&["git", "status", "--porcelain"]);
    let status: Vec<&str> = status.trim_end().split('\n').collect();

    let mut files: Vec<File> = vec![];

    for file in status {
        if let Some(path) = file.split_whitespace().last() {
            files.push(File {
                path: String::from(path),
                full_str: String::from(file),
                is_tracked: !file.starts_with(' ') && !file.starts_with('?'),
            });
        }
    }

    files
}

fn draw_stage_selection() -> io::Result<()> {
    let mut stdout = io::stdout();
    let mut pointer: usize = 0;

    execute!(stdout, cursor::Hide)?;

    loop {
        let mut options = parse_porcelain();
        if options.len() == 0 {
            return Err(io::Error::new(io::ErrorKind::Other, "No Files to stage"));
        }

        for (i, op) in options.iter().enumerate() {
            queue!(
                stdout,
                Clear(ClearType::CurrentLine),
                SetForegroundColor(Color::Blue),
                Print(if i == pointer { "> " } else { "  " }),
                if op.is_tracked {
                    SetForegroundColor(Color::Green)
                } else {
                    SetForegroundColor(Color::Red)
                },
                Print(&op.full_str),
                ResetColor,
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
                KeyCode::Char('q') | KeyCode::Esc => {
                    execute!(stdout, cursor::RestorePosition, cursor::Show)?;
                    return Ok(());
                }
                KeyCode::Char('a') => {
                    for mut file in options {
                        file.toggle_stage();
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if pointer + 1 < options.len() {
                        pointer += 1;
                    }
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if pointer > 0 {
                        pointer -= 1;
                    }
                }
                KeyCode::Enter | KeyCode::Char(' ') => {
                    if let Some(file) = options.get_mut(pointer) {
                        file.toggle_stage();
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn draw() -> io::Result<()> {
    enable_raw_mode()?;
    draw_stage_selection()?;
    disable_raw_mode()?;
    Ok(())
}
