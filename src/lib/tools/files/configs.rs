use crate::tools::files::configs::FilesError::{FileNotFound, FolderNotFound};
use std::path::PathBuf;
use std::{
    fs::{self},
    io,
    path::Path,
};

#[derive(thiserror::Error, Debug)]
pub enum FilesError {
    #[error("folder not exists")]
    FolderNotFound,
    #[error("file not exists")]
    FileNotFound,
    #[error("error while retrieving lib.data.files.files: {0}")]
    Error(String),
}

impl From<io::Error> for FilesError {
    fn from(err: io::Error) -> Self {
        Self::Error(err.to_string())
    }
}

pub fn get_cfg(path: &str) -> Result<PathBuf, FilesError> {
    let file = PathBuf::from(path);
    if file.is_file() {
        return Ok(file);
    }

    if file.is_dir() {
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
                // println!("{:?}", entry.path()) // uncomment to check all files
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
    if input.is_empty() {
        None
    } else {
        Some(input.parse::<usize>().expect(""))
    }
}
