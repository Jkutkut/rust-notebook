use serde::{Serialize, Deserialize};
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
    notes.insert(String::from("Obj1"), NotebookEntry {
        name: String::from("Obj1"),
        description: String::from("desc obj1"),
    });
    notes.insert(String::from("Obj2"), NotebookEntry {
        name: String::from("Obj2"),
        description: String::from("desc obj1"),
    });
    notes.insert(String::from("Obj3"), NotebookEntry {
        name: String::from("Obj3"),
        description: String::from("desc obj1"),
    });
    notes.insert(String::from("Obj4"), NotebookEntry {
        name: String::from("Obj4"),
        description: String::from("desc obj1"),
    });
    // TODO implement
    let previous: String = fs::read_to_string(filename).unwrap_or(String::new());
    if previous.len() == 0 {
        print!(" No session found\n");
        return;
    }
    // let parse = json::parse(previous);
    // for n in parse["notes"] {
    //     print!("Note: {{name: {}, desc: {}}}\n", n["name"], n["description"]);
    // }
    print!(" Session (NOT) restored. Yet.\n"); // TODO change
}
