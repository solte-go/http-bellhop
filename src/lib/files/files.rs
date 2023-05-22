use crate::files::files::FilesError::{FileNotFound, FolderNotFound};
use std::fmt::Error;
use std::path::PathBuf;
use std::{
    fs::{self, DirEntry},
    io,
    path::Path,
};

const CONFIG_DIR: &str = "./requests/";

#[derive(thiserror::Error, Debug)]
pub enum FilesError {
    #[error("folder not exists")]
    FolderNotFound,
    #[error("file not exists")]
    FileNotFound,
    #[error("error while retrieving files: {0}")]
    Error(String),
}

impl From<io::Error> for FilesError {
    fn from(err: io::Error) -> Self {
        Self::Error(err.to_string())
    }
}

pub fn get_cfg(file: &str) -> Result<PathBuf, FilesError> {
    // let path = format!("{}{}", CONFIG_DIR, file);
    let file = PathBuf::from(file);
    if file.is_file() {
        return Ok(file);
    }
    Err(FileNotFound)
}

pub fn get_files(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), FilesError> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                get_files(&path, files)?;
            } else {
                files.push(entry.path());
                println!("{:?}", entry.path())
            }
        }
    }
    Ok(())
}

pub fn get_input() -> Option<usize> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please pick one of displayed options");
    }
    let input = buffer.trim().to_lowercase().to_owned();
    if &input == "" {
        None
    } else {
        Some(input.parse::<usize>().expect(""))
    }
}
