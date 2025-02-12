use std::error::Error;

use local_watcher::local_watcher::watch_dir;

mod local_watcher;
mod notification;

fn main() -> Result<(), Box<dyn Error>> {
    watch_dir(".test", |event| println!("event: {:?}", event))?;
    Ok(())
}
