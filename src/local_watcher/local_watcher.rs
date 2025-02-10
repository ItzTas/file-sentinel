use notify::RecommendedWatcher;
pub use notify::{RecursiveMode, Watcher};
use std::{path::Path, sync::mpsc};

fn watch_dir(dir_str: &str) -> Result<(), notify::Error> {
    let (tx, rx) = mpsc::channel();

    let mut watcher: RecommendedWatcher = notify::recommended_watcher(tx)?;

    watcher.watch(Path::new(dir_str), RecursiveMode::Recursive)?;
    for res in rx {
        match res {
            Ok(event) => println!("event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}
