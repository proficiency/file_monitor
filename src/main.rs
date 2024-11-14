mod monitor;
mod internal;
mod cache;
mod test;

use crate::monitor::*;
use crate::test::*;

use clap::Parser;
use std::path::PathBuf;

use tokio::{
    signal,
    task,
};

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    /// The directory to monitor, e.g. "inbox" or "./inbox"
    #[arg(short, long, default_value = "inbox")]
    directory: PathBuf,

    /// Runs an automated testsuite. Conflicts with -d/--directory
    #[arg(short, long, conflicts_with = "directory")]
    test: bool,
}

#[tokio::main]
async fn main() -> notify::Result<()> {
    let args = Args::parse();
    let path = args.directory;
    if !std::fs::exists(path.to_str().expect("please provide a path consisting of utf-8 text"))? {
        std::fs::create_dir(path.clone())?
    }

    println!("spawning monitor for \"{}\"...", path.display());

    let mut monitor = Monitor::new(PathBuf::from(path))?;
    monitor.print_cache();

    if args.test {
        let test_task = task::spawn(run_tests());
        tokio::select! {
            result = monitor.async_monitor() => {
                if let Err(e) = result {
                    eprintln!("[monitor] error: {e}");
                }
            }

            _ = test_task => {
                monitor.print_cache();
            }
        }

        return Ok(());
    } else {
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
    }

    Ok(())
}