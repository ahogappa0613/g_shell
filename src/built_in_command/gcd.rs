use std::path::Path;
use std::{borrow::Borrow, fs};
use std::{env, path::PathBuf};
use std::{
    io::{stdin, stdout, Stdout, Write},
    ops::Index,
};
use termion::*;
use termion::{
    cursor::DetectCursorPos,
    raw::{IntoRawMode, RawTerminal},
};

//use std::io::{self, BufRead, BufReader};

use dirs;
use termion::input::TermRead;

use super::super::parser::parser::CommandParse;

struct DirPathData {
    dirs: Vec<PathBuf>,
}

impl DirPathData {
    pub fn new() -> Self {
        Self { dirs: vec![] }
    }
}

pub fn run_gcd(commands: &CommandParse) -> Result<(), String> {
    let path = if commands.get_path().trim().is_empty() {
        "."
    } else {
        commands.get_path()
    };
    let mut dirs = map_dir(path);
    let mut index = 0;

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let (x, y) = stdout.cursor_pos().unwrap();

    ls_dirs_with_index(&dirs, index, &mut stdout);
    for c in stdin.keys() {
        write!(stdout, "{}{}", cursor::Goto(x, y), clear::AfterCursor).unwrap();
        stdout.flush().unwrap();
        match c {
            Ok(event::Key::Up) => {
                if index == 0 {
                    index = dirs.len() - 1;
                } else {
                    index -= 1;
                }
                ls_dirs_with_index(&dirs, index, &mut stdout);
            }

            Ok(event::Key::Down) => {
                if (index + 1) > dirs.len() - 1 {
                    index = 0;
                } else {
                    index += 1;
                }
                ls_dirs_with_index(&dirs, index, &mut stdout);
            }

            Ok(event::Key::Right) => {
                let current_path = dirs[index].clone();
                dirs = map_dir(dirs[index].to_str().unwrap());
                if dirs.len() == 0 {
                    env::set_current_dir(current_path);
                    return Ok(());
                }
                index = 0;
                ls_dirs_with_index(&dirs, index, &mut stdout);
            }

            Ok(event::Key::Left) => {
                let mut cuurent_path = dirs[index].clone();
                cuurent_path.pop();
                cuurent_path.pop();
                dirs = map_dir(cuurent_path.to_str().unwrap());
                index = 0;
                ls_dirs_with_index(&dirs, index, &mut stdout);
            }

            // tab key
            Ok(event::Key::Char('\t')) => {}

            // return key
            Ok(event::Key::Char('\n')) => {
                env::set_current_dir(&dirs.index(index));
                return Ok(());
            }
            Ok(event::Key::Ctrl('c')) => break,
            _ => {}
        }
    }

    Ok(())
}

fn map_dir(path: &str) -> Vec<PathBuf> {
    fs::read_dir(path)
        .unwrap()
        .filter(|entry| match entry {
            Ok(entry) => entry.metadata().unwrap().is_dir(),
            Err(_) => false,
        })
        .map(|dir| dir.unwrap().path())
        .collect::<Vec<PathBuf>>()
}

fn ls_dirs_with_index(dirs: &Vec<PathBuf>, index: usize, stdout: &mut RawTerminal<Stdout>) {
    for (i, dir) in dirs.iter().enumerate() {
        if index == i {
            write!(stdout, "> {}\n\r", dir.to_str().unwrap()).unwrap();
        } else {
            write!(stdout, "{}\n\r", dir.to_str().unwrap()).unwrap();
        }
    }

    stdout.flush().unwrap();
}
