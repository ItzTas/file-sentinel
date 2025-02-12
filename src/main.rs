use local_watcher::local_watcher::watch_dir;
use std::error::Error;

mod local_watcher;

fn main() -> Result<(), Box<dyn Error>> {
    watch_dir(".gitignore")?;
    Ok(())
}
