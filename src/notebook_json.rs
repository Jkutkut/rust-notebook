use HashMap;
use crate::notebook::NotebookEntry;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn notebook_save(filename: &str, notes: &HashMap<String, NotebookEntry>) {
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
    print!("File:\n{data}\n");
    print!("Saving session on {filename}...");
    std::fs::write(
        filename,
        data,
    ).unwrap();
    print!(" Session saved!\n");
}
