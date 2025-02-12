use notify::{Error, ErrorKind, Event, RecommendedWatcher};
pub use notify::{RecursiveMode, Result, Watcher};
use std::{fs::canonicalize, path::Path, sync::mpsc};

pub fn watch_dir<F>(dir_str: &str, on_dir_change: F) -> Result<()>
where
    F: Fn(Event) -> (),
{
    let (tx, rx) = mpsc::channel::<Result<Event>>();

    let mut watcher: RecommendedWatcher = notify::recommended_watcher(tx)?;

    let path = Path::new(dir_str);

    let absolute_path = canonicalize(path)?;
    if !path.exists() {
        return Err(Error::new(ErrorKind::PathNotFound));
    }

    watcher.watch(&absolute_path, RecursiveMode::Recursive)?;
    for res in rx {
        match res {
            Ok(event) => on_dir_change(event),
            Err(e) => return Err(e),
        }
    }

    Ok(())
}
