use crate::{commands, git_commands::git, tui::select};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io;
use std::vec;

pub struct File {
    pub path: String,
    pub full_str: String,
    pub is_tracked: bool,
}

pub struct Status {
    pub staged: Vec<String>,
    pub unstaged: Vec<String>,
}

impl File {
    fn toggle(&mut self) {
        if self.is_tracked {
            git::unstage(vec![self.path.clone()]).unwrap();
        } else {
            git::stage(vec![self.path.clone()]);
        }
        self.is_tracked = !self.is_tracked;
    }
}

pub fn parse_status() -> Vec<File> {
    let status = commands::command_output(&["git", "status", "--porcelain", "-uall"]);
    let status: Vec<&str> = status.trim_end().split('\n').collect();

    let mut files: Vec<File> = vec![];

    for file in status {
        if let Some(path) = file.split_whitespace().last() {
            files.push(File {
                path: path.into(),
                full_str: file.into(),
                is_tracked: !file.starts_with(' ') && !file.starts_with('?'),
            });
        }
    }

    files
}

fn draw_stage_selection() -> io::Result<()> {
    let mut selector: select::Selector<File> = select::new(vec![]);

    loop {
        selector.options = parse_status();
        if selector.options.is_empty() {
            return Err(io::Error::new(io::ErrorKind::Other, "No Files to stage"));
        }

        match selector.draw()? {
            Some(select::SelectVal::KeyboardInterrupt) => {
                return Err(io::Error::new(io::ErrorKind::Other, "Keyboard Interrupt"));
            }
            Some(select::SelectVal::Quit) => {
                return Ok(());
            }
            Some(select::SelectVal::All) => {
                for file in selector.options.iter_mut() {
                    file.toggle();
                }
            }
            Some(select::SelectVal::Select) => {
                if let Some(file) = selector.options.get_mut(selector.pointer) {
                    file.toggle();
                    selector.move_down();
                }
            }
            _ => {}
        }
    }
}

pub fn draw() -> io::Result<()> {
    enable_raw_mode()?;
    draw_stage_selection()?;
    disable_raw_mode()?;
    Ok(())
}
