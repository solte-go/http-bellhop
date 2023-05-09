use std::{
    fs::{self, DirEntry},
    io,
    path::Path,
};

pub fn get_files(dir: &Path, files: &mut Vec<DirEntry>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let new_entry = entry?;
            let path = new_entry.path();
            if path.is_dir() {
                get_files(&path, files)?;
            } else {
                files.push(new_entry);
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
