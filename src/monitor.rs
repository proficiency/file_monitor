use crate::cache::*;

use std::path::PathBuf;

use notify::{
    Config,
    Event,
    ReadDirectoryChangesWatcher,
    RecommendedWatcher,
    RecursiveMode,
    Watcher,
};

use tokio::sync::mpsc::{
    channel,
    Receiver
};

pub struct Monitor {
    directory: PathBuf,
    cache: Cache,
    watcher: ReadDirectoryChangesWatcher,
    rx: Receiver<notify::Result<Event>>,
}

impl Monitor {
    pub fn new(directory: PathBuf) -> notify::Result<Self> {
        let (tx, rx) = channel(1);

        let watcher = RecommendedWatcher::new(move |res| {
            tx.blocking_send(res).unwrap();
        }, Config::default())?;


        Ok(Self {
            cache: Cache::new(&directory)?,
            directory,        
            watcher,
            rx,
        })
    }

    pub async fn async_monitor(&mut self) -> notify::Result<()> {
        self.watcher.watch(&self.directory, RecursiveMode::Recursive)?;

        while let Some(res) = self.rx.recv().await {
            match res {
                Ok(event) => self.cache.handle_event(event)?,
                Err(e) => eprintln!("failed to receive event {:?}", e),
            }
        }

        Ok(())
    }

    pub fn print_cache(&self) {
        self.cache.print();
    }
}