mod monitor;
mod internal;
mod cache;
mod tests;

use crate::monitor::*;

use clap::Parser;
use std::path::PathBuf;

use tokio::signal;

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    /// The directory to monitor, e.g. "inbox" or "./inbox"
    #[arg(short, long, default_value = "inbox")]
    directory: PathBuf,
}

#[tokio::main]
async fn main() -> notify::Result<()> {
    let args = Args::parse();
    let path = args.directory;
    if !std::fs::exists(path.to_str().expect("please provide a path consisting of utf-8 text"))? {
        std::fs::create_dir(path.clone())?
    }

    println!("spawning monitor for \"{}\"...", path.display());

    let mut monitor = Monitor::new(path)?;
    monitor.print_cache();

    tokio::select! {
            result = monitor.async_monitor() => {
                if let Err(e) = result {
                    eprintln!("[monitor] error: {e}");
                }
            }

            _ = signal::ctrl_c() => {
                monitor.print_cache();
            }
        }

    Ok(())
}
