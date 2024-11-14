use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use chrono::{
    offset::Local,
    DateTime,
};

pub(crate) fn get_date_modified(path: &PathBuf) -> Result<DateTime<Local>, Error> {
    if let Ok(metadata) = std::fs::metadata(&path) {
        if let Ok(modified) = metadata.modified() {
           return Ok(modified.into())
        }
    }

    Err(Error::new(ErrorKind::InvalidInput, format!("failed to get \"Date Modified\" for {}. the file likely does not exist", path.display())))
}

pub(crate) fn get_relative_path(abs_path: &PathBuf) -> Result<PathBuf, Error> {
    let mut current_dir = std::env::current_dir()?;
    let mut file_path = abs_path.clone();
    if file_path.exists() {
        file_path = file_path.canonicalize()?;
        current_dir = current_dir.canonicalize()?;
    }

    if let Ok(relative_path) = file_path.strip_prefix(&current_dir) {
        Ok(relative_path.to_path_buf())
    } else {
        Ok(file_path.to_path_buf())
    }
}