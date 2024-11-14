use std::io::Error;
use std::path::{Path, PathBuf};

use chrono::{
    offset::Local,
    DateTime,
};

pub(crate) fn get_date_modified(path: &Path) -> Result<DateTime<Local>, Error> {
    Ok(std::fs::metadata(path)?.modified()?.into())
}

pub(crate) fn get_relative_path(mut abs_path: PathBuf) -> Result<PathBuf, Error> {
    let mut current_dir = std::env::current_dir()?;
    if abs_path.exists() {
        abs_path = abs_path.canonicalize()?;
        current_dir = current_dir.canonicalize()?;
    }

    if let Ok(relative_path) = abs_path.strip_prefix(&current_dir) {
        Ok(relative_path.to_path_buf())
    } else {
        Ok(abs_path)
    }
}