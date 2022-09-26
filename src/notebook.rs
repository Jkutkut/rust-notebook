use std::collections::HashMap;

use crate::shell::shell_handler::FtDictEntry;
use crate::shell::nanoshell::Nanoshell;

use crate::notebook_json::notebook_save;

pub fn notebook_cmds() -> Box<[FtDictEntry]> {
    Box::new([
        FtDictEntry {
            name: String::from("list"),
            man: String::from("Show all the elements."),
        },
        FtDictEntry {
            name: String::from("add"),
            man: String::from("Opens a form to fill all the data."),
        },
        FtDictEntry {
            name: String::from("remove"),
            man: String::from("Removes a element by name."),
        },
    ])
}

// NotebookEntry
pub struct NotebookEntry {
    pub name: String,
    pub description: String,
}

// Notebook
pub struct Notebook<'a> {
    file: &'a str,
    notes: HashMap<String, NotebookEntry>,
    shell: Nanoshell<'a>,
}

impl Notebook<'_> {
    pub fn init(&self) {
        // TODO recover session
    }

    pub fn run(&mut self) {
        self.shell.init();
        let mut cmd: String;
        loop {
            cmd = self.shell.run();
            match cmd.as_str() {
                "exit" => break,
                "list" => self.list(),
                "add" => self.add(),
                "remove" => self.remove(),
                &_ => todo!(),
            }
        }
        self.save_session();
        self.shell.print("Exiting Notebook\n");
    }

    // Commands

    fn list(&self) {
        self.shell.print_buffered("List:\n--------------------------\n");
        for (_, entry) in &self.notes {
            self.shell.print_buffered("- ");
            self.shell.print_buffered(&entry.name);
            self.shell.print_buffered("\n  ");
            self.shell.print_buffered(&entry.description);
            self.shell.print_buffered("\n");
        }
        self.shell.print("--------------------------\n");
    }

    fn add(&mut self) {
        let name: String = self.shell.ask("Name:\n  ");
        let description: String = self.shell.ask("Description:\n  ");

        self.notes.insert(
            String::from(&name),
            NotebookEntry {
                name: name,
                description: description,
            },
        );
    }

    fn remove(&mut self) {
        self.shell.print("Name:\n  ");
        let name: String = self.shell.get_input();

        if !self.notes.contains_key(&name) {
            self.shell.print_buffered("Note '");
            self.shell.print_buffered(&name);
            self.shell.print("' does not exists.\n");
        }
        else {
            self.notes.remove(&name);
            self.shell.print_buffered(&name);
            self.shell.print(" removed.\n");
        }
    }

    // Session
    fn save_session(&self) {
        notebook_save(self.file, &self.notes);
    }
}
