use HashMap;
use crate::notebook::NotebookEntry;

pub fn notebook_save(filename: &str, notes: &HashMap<String, NotebookEntry>) {
    // TODO
    print!("Saving session on {filename}...");
    std::fs::write(
        filename,
        "This is a file",
    ).unwrap();
    print!(" Session saved!\n");
}
