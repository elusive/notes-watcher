mod watcher;

fn main() {
    // notes directory
    let notesPath = "/d/Archive/Notes/journal/";

    println!("Hello Rust Watcher");

    watcher::startWatch(notesPath);
}
