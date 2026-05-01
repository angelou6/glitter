use std::io::{self, Write};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute, queue,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal,
};

use crate::commands::{run_command, run_command_output};

struct File {
    location: String,
    status: char,
    is_staged: bool,
}

pub struct Interface {
    files: Vec<File>,
    cursor: usize,
}

fn stage(location: &str) {
    run_command(&["git", "add", location]);
}

fn unstage(location: &str) {
    run_command(&["git", "restore", "--staged", location]);
}

fn status_color(status: &str) -> Color {
    match status.trim() {
        "M" => Color::Yellow,   // modified
        "A" => Color::Green,    // added
        "D" => Color::Red,      // deleted
        "R" => Color::Magenta,  // renamed
        "?" => Color::DarkGrey, // untracked
        _ => Color::White,
    }
}

impl Interface {
    pub fn new() -> Result<Interface, String> {
        let mut inter = Interface {
            files: vec![],
            cursor: 0,
        };
        inter.update()?;
        Ok(inter)
    }

    fn update(&mut self) -> Result<(), String> {
        let status = run_command_output(&["git", "status", "--porcelain"]);
        if status.is_empty() {
            return Err(String::from("There is no staged or unstaged files."));
        }

        let mut files: Vec<File> = Vec::new();
        for entry in status.split('\n') {
            let location = entry.split(' ').last().ok_or("Result unkown")?;

            match entry.get(0..2) {
                Some(status) => {
                    if status.contains('?') {
                        files.push(File {
                            location: location.to_owned(),
                            status: '?',
                            is_staged: false,
                        });
                    } else {
                        let occupied_index = status.chars().position(|c| c != ' ').unwrap();
                        let status_char = status.chars().nth(occupied_index).unwrap();
                        files.push(File {
                            location: location.to_owned(),
                            status: status_char,
                            is_staged: occupied_index == 0,
                        });
                    }
                }
                None => continue,
            }
        }
        self.files = files;
        if !self.files.is_empty() && self.cursor >= self.files.len() {
            self.cursor = self.files.len() - 1;
        }

        self.files.sort_by(|a, b| {
            if a.is_staged == b.is_staged {
                std::cmp::Ordering::Equal
            } else if a.is_staged {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            }
        });

        Ok(())
    }

    fn print_status(&mut self) -> io::Result<()> {
        let mut out = io::stdout();

        execute!(
            out,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0)
        )?;

        // unstaged
        queue!(
            out,
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Yellow),
            Print("  unstaged\n\r"),
            ResetColor,
            SetAttribute(Attribute::Reset),
            Print(format!("  {}\n\r", "─".repeat(36))),
        )?;

        for (i, f) in self.files.iter().enumerate().filter(|(_, f)| !f.is_staged) {
            let status_color = status_color(&f.status.to_string());
            let prefix = if i == self.cursor { ">" } else { " " };

            queue!(
                out,
                SetForegroundColor(Color::Cyan),
                Print(format!("  {} ", prefix)),
                ResetColor
            )?;
            queue!(
                out,
                SetForegroundColor(status_color),
                SetAttribute(Attribute::Bold),
                Print(format!("{:<2} ", f.status)),
                ResetColor,
                SetAttribute(Attribute::Reset)
            )?;
            queue!(out, Print(format!("{}\n\r", f.location)))?;
        }

        if self.files.iter().all(|f| f.is_staged) {
            queue!(
                out,
                SetForegroundColor(Color::DarkGrey),
                Print("  (empty)\n\r"),
                ResetColor
            )?;
        }

        // staged
        queue!(
            out,
            Print("\n\r"),
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Green),
            Print("  staged\n\r"),
            ResetColor,
            SetAttribute(Attribute::Reset),
            Print(format!("  {}\n\r", "─".repeat(36))),
        )?;

        for (i, f) in self.files.iter().enumerate().filter(|(_, f)| f.is_staged) {
            let prefix = if i == self.cursor { ">" } else { " " };

            queue!(
                out,
                SetForegroundColor(Color::Cyan),
                Print(format!("  {} ", prefix)),
                ResetColor
            )?;
            queue!(
                out,
                SetForegroundColor(Color::Green),
                SetAttribute(Attribute::Bold),
                Print(format!("{:<2} ", f.status)),
                ResetColor,
                SetAttribute(Attribute::Reset)
            )?;
            queue!(out, Print(format!("{}\n\r", f.location)))?;
        }

        if self.files.iter().all(|f| !f.is_staged) {
            queue!(
                out,
                SetForegroundColor(Color::DarkGrey),
                Print("  (empty)\n\r"),
                ResetColor
            )?;
        }

        out.flush()
    }

    pub fn draw(&mut self) -> io::Result<()> {
        let mut stdout = io::stdout();

        terminal::enable_raw_mode()?;
        execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

        loop {
            self.print_status()?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char('j') | KeyCode::Down => {
                        if self.cursor + 1 < self.files.len() {
                            self.cursor += 1;
                        }
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        if self.cursor > 0 {
                            self.cursor -= 1;
                        }
                    }
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        let location = self.files[self.cursor].location.clone();
                        let is_staged = self.files[self.cursor].is_staged;
                        if is_staged {
                            unstage(&location);
                        } else {
                            stage(&location);
                        }
                        self.update().unwrap();
                    }
                    _ => {}
                }
            }
        }

        terminal::disable_raw_mode()?;
        execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;

        Ok(())
    }
}
