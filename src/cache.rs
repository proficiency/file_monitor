use crate::internal::*;

use std::{
    collections::HashMap,
    io::Error,
    path::PathBuf,
};

use walkdir::WalkDir;

use notify::{
    event::ModifyKind,
    event::RenameMode,
    Event,
    EventKind,
};

use chrono::{
    offset::Local,
    DateTime,
};

pub struct Cache {
    cache: HashMap<PathBuf, DateTime<Local>>,
}

impl Cache {
    pub fn new(dir: &PathBuf) -> Result<Self, Error> {
        let mut cache: HashMap<PathBuf, DateTime<Local>> = HashMap::new();
        for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
            if entry.path() == dir {
                continue;
            }

            let path = &get_relative_path(&PathBuf::from(entry.path()))?;
            if !cache.contains_key(path) {
                cache.insert(path.clone(), get_date_modified(path)?);
            }
        }

        Ok(Self {
            cache,
        })
    }

    pub fn handle_event(&mut self, event: Event) -> notify::Result<()> {
        let path = &get_relative_path(event.paths.first().expect("no path associated with the operation."))?;

        let operation_type = match &event.kind {
            EventKind::Create(_) => {
                self.cache.insert(path.clone(), get_date_modified(path)?);
                "NEW"
            }

            EventKind::Remove(_) => {
                self.cache.remove(path);
                "DEL"
            }

            EventKind::Modify(modify_kind) => {
                match modify_kind {
                    ModifyKind::Name(rename_mode) => {
                        match rename_mode {
                            RenameMode::To => {
                                self.cache.insert(path.clone(), Local::now());
                            }

                            RenameMode::From => {
                                self.cache.remove(path);
                            }

                            _ => {}
                        }
                    }

                    ModifyKind::Data(_) | ModifyKind::Metadata(_) | ModifyKind::Other => {
                        self.cache.insert(path.clone(), Local::now());
                    }

                    _ => {}
                }

                "MOD"
            }

            _ => { "" }
        };

        if !operation_type.is_empty() {
            println!("[{}] {}", operation_type, path.display());
        }

        Ok(())
    }

    pub fn print(&self) {
        for (path, modified) in &self.cache {
            println!("[{}] {}", modified.to_rfc2822(), path.display());
        }
    }
}