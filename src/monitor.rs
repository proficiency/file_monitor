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

use futures::{
    channel::mpsc::{
        channel,
        Receiver
    },

    SinkExt,
    StreamExt,
};

pub struct Monitor {
    directory: PathBuf,
    cache: Cache,
    watcher: ReadDirectoryChangesWatcher,
    rx: Receiver<notify::Result<Event>>,
}

impl Monitor {
    pub fn new(directory: PathBuf) -> notify::Result<Self> {
        let (mut tx, rx) = channel(1);

        let watcher = RecommendedWatcher::new(move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        },
                                              Config::default(),
        )?;


        Ok(Self {
            directory: directory.clone(),
            cache: Cache::new(&directory)?,
            watcher,
            rx,
        })
    }

    pub async fn async_monitor(&mut self) -> notify::Result<()> {
        self.watcher.watch(self.directory.as_ref(), RecursiveMode::Recursive)?;

        while let Some(res) = self.rx.next().await {
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