use std::path::PathBuf;

mod watcher;

fn main() {
    // notes directory
    let notes_path = "/c/work/Notes/journal/";

    println!("Hello Rust Watcher");

    watcher::start_watch(PathBuf::from(notes_path));
}
