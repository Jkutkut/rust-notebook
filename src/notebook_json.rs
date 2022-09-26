use HashMap;
use std::fs;

use crate::notebook::NotebookEntry;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn notebook_save(filename: &str, notes: &HashMap<String, NotebookEntry>) {
    print!("Saving session on {filename}...");
    fn add_note2json(mut n: String, note: &NotebookEntry, first: bool) -> String {
        if !first {
            n = n + ",";
        }
        n = n + "{\"name\":\"";
        n = n + &note.name;
        n = n + "\",\"description\":\"";
        n = n + &note.description;
        n = n + "\"}";
        n
    }

    let mut data = String::from("{\"version\":\"");
    data = data + VERSION;
    data = data + "\",\"data\":[";
    if notes.keys().len() > 0 {
        let mut first: bool = true;
        for (_, entry) in notes {
            data = add_note2json(data, &entry, first);
            if first {
                first = false;
            }
        }
    }
    data = data + "]}\n";
    std::fs::write(
        filename,
        data,
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
