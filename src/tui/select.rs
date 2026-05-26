use std::io::{self, Write};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute, queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};

use crate::stage;

pub enum SelectVal {
    KeyboardInterrupt,
    Quit,
    All,
    Select,
}

pub trait DrawOption {
    fn draw_option(&self, stdout: &mut io::Stdout) -> io::Result<()>;
}

impl DrawOption for String {
    fn draw_option(&self, stdout: &mut io::Stdout) -> io::Result<()> {
        queue!(stdout, Clear(ClearType::UntilNewLine), Print(self))?;
        Ok(())
    }
}

impl DrawOption for stage::File {
    fn draw_option(&self, stdout: &mut io::Stdout) -> io::Result<()> {
        let color = if self.is_tracked {
            Color::Green
        } else {
            Color::Red
        };

        queue!(
            stdout,
            SetForegroundColor(color),
            Clear(ClearType::UntilNewLine),
            Print(&self.full_str),
            ResetColor
        )?;

        Ok(())
    }
}

pub struct Selector<T: DrawOption> {
    pub options: Vec<T>,
    pub pointer: usize,
    stdout: io::Stdout,
}

pub fn new<T: DrawOption>(options: Vec<T>) -> Selector<T> {
    Selector {
        options,
        pointer: 0,
        stdout: io::stdout(),
    }
}

impl<T: DrawOption> Drop for Selector<T> {
    fn drop(&mut self) {
        let _ = execute!(self.stdout, cursor::RestorePosition, cursor::Show);
    }
}

impl<T: DrawOption> Selector<T> {
    fn draw_options(&mut self) -> io::Result<()> {
        for (i, op) in self.options.iter().enumerate() {
            queue!(
                self.stdout,
                SetForegroundColor(Color::Blue),
                Print(if i == self.pointer { "> " } else { "  " }),
                ResetColor,
            )?;
            op.draw_option(&mut self.stdout)?;
            queue!(self.stdout, Print("\r\n"))?;
        }

        queue!(
            self.stdout,
            cursor::SavePosition,
            cursor::MoveToPreviousLine(self.options.len() as u16)
        )?;

        self.stdout.flush()?;
        Ok(())
    }

    pub fn move_down(&mut self) {
        self.pointer = (self.pointer + 1) % self.options.len()
    }

    pub fn move_up(&mut self) {
        self.pointer = (self.pointer - 1) % self.options.len()
    }

    pub fn draw(&mut self) -> io::Result<Option<SelectVal>> {
        execute!(self.stdout, cursor::Hide)?;
        self.draw_options()?;

        if let Event::Key(key) = event::read()? {
            match (key.code, key.modifiers) {
                (KeyCode::Char('c'), KeyModifiers::CONTROL)
                | (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                    return Ok(Some(SelectVal::KeyboardInterrupt));
                }
                (KeyCode::Char('q'), _) | (KeyCode::Esc, _) => {
                    return Ok(Some(SelectVal::Quit));
                }
                (KeyCode::Char('a'), _) => {
                    return Ok(Some(SelectVal::All));
                }
                (KeyCode::Down, _) => self.move_down(),
                (KeyCode::Up, _) => self.move_up(),
                (KeyCode::Enter, _) | (KeyCode::Char(' '), _) => {
                    return Ok(Some(SelectVal::Select));
                }
                _ => {}
            }
        }
        Ok(None)
    }
}
