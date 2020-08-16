use notify::{watcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

pub use notify::Error;
pub use notify::RawEvent as Event;

pub struct WatcherInstance

pub fn startWatch(path_name: &str) {
    // create a channel to receive the events
    let (tx, rx) = channel();

    // create a watcher object, delivering debounced events
    // the notification back-end is selected based on the platform
    let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();

    // add the path to be watched. all files and folders at that path
    // and below will be monitored for changes.
    watcher.watch(path_name, RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(event) => println!("{:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

pub fn stopWatch(path_name: &str) {
    
}
