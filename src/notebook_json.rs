use serde::{Serialize, Deserialize};
use serde_json::Result;

use HashMap;
use std::fs;

use crate::notebook::NotebookEntry;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn notebook_hashm2vec(notes: &HashMap<String, NotebookEntry>) -> Vec<NotebookEntry> {
    let len = notes.keys().len();
    let mut n = Vec::with_capacity(len);
    for (_, entry) in notes {
        n.push(NotebookEntry {
            name: String::from(&entry.name),
            description: String::from(&entry.description),
        });
    }
    n
}

#[derive(Serialize, Deserialize)]
struct NotebookJSON {
    version: String,
    notes: Vec<NotebookEntry>,
}

pub fn notebook_save(filename: &str, notes: &HashMap<String, NotebookEntry>) {
    print!("Saving session on {filename}...");
    let data: NotebookJSON = NotebookJSON {
        version: String::from(VERSION),
        notes: notebook_hashm2vec(notes),
    };
    let content = serde_json::to_string(&data).unwrap();
    std::fs::write(
        filename,
        content,
    ).unwrap();
    print!(" Session saved!\n");
}

pub fn notebook_load(filename: &str, notes: &mut HashMap<String, NotebookEntry>) {
    print!("Restoring previous session...");
    let previous: String = fs::read_to_string(filename).unwrap_or(String::new());
    if previous.len() == 0 {
        print!(" No session found\n");
        return;
    }
    let jresult: Result<NotebookJSON> = serde_json::from_str(&previous);

    match jresult {
        Err(e) => {
            print!("\nNot able to restore the session. There was an error with the file\n");
            print!("Error:\n  {:?}\n", e);
            print!("File:\n  {previous}\n");
        },
        Ok(notes_json) => {
            print!(" (v{})", notes_json.version);
            for n in notes_json.notes {
                notes.insert(
                    String::from(&n.name),
                    n
                );
            }
            print!(" Session restored.\n");
        },
    };
}
