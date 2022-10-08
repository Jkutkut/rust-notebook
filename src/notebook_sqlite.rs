use std::fs;
use std::io;
use std::io::Write;

pub struct NotebookDB {
    db: sqlite::Connection
}

impl NotebookDB {
    pub fn new(file: &str) -> Self {
        NotebookDB {
            db: NotebookDB::new_db_connection(file)
        }
    }

    fn is_valid_table(&self, table_type: &str, check_note: bool) -> bool{
        match table_type {
            "category" | "tag" => true,
            "note" => check_note,
            _ => false
        }
    }

    // List

    pub fn list_all(&self, table_type: &str) -> Result<&str, String> {
        // TODO
        Err(String::from("Not implemented"))
    }

    pub fn list(&self, table_type: &str, t: &str) -> Result<&str, String> {
        let mut statement;
        match table_type {
            "category" => {
                statement = self.db.prepare("
                    SELECT N.NOTE_NAME AS 'Name', N.NOTE_DESC AS 'Description',
                        C.CAT_NAME AS 'Category', T.TAG_NAME as 'Tag'
                    FROM NOTE N, CATEGORY C, TAG T
                    WHERE N.CATEGORY_ID == C.ID and T.ID = N.TAG_ID and
                    UPPER(C.CAT_NAME) == UPPER(?);
                ").unwrap().bind(1, t).unwrap();
            },
            "tag" => {
                statement = self.db.prepare("
                    SELECT N.NOTE_NAME AS 'Name', N.NOTE_DESC AS 'Description',
                        C.CAT_NAME AS 'Category', T.TAG_NAME as 'Tag'
                    FROM NOTE N, CATEGORY C, TAG T
                    WHERE N.CATEGORY_ID == C.ID and T.ID = N.TAG_ID and
                    UPPER(T.TAG_NAME) == UPPER(?);
                ").unwrap().bind(1, t).unwrap();
            }
            _ => return Err(String::from("Use category or tag")),
        }
        print!("Listing {table_type} {t}\n");
        while let Ok(sqlite::State::Row) = statement.next() {
            print!("-----------\n");
            print!(
                "{}:     c: {} t: {}\n\n{}\n",
                statement.read::<String>(0).unwrap(), // Name
                statement.read::<String>(2).unwrap(), // Category
                statement.read::<String>(3).unwrap(), // Tag
                statement.read::<String>(1).unwrap() // Description
            );
            print!("-----------\n");
        }
        Err(String::from("Not fully implemented"))
    }

    // 
    pub fn add(&self, category: &String) {
        // TODO
        print!("Adding a {category}\n");
    }

    pub fn remove(&self, category: &String) {
        // TODO
        print!("Removing {category}\n");
    }
}

// NotebookDB tools
impl NotebookDB {
    pub fn new_db_connection(file: &str) -> sqlite::Connection {
        let f = String::from(file);
        match std::path::Path::new(file).exists() {
            false => {
                print!("Creating DB...");
                let db = sqlite::open(file).unwrap();
                match fs::read_to_string("docs/db.sql") {
                    Err(e) => panic!("\nNot able to read the script:\n{}", e.to_string()),
                    Ok(script) => {
                        match db.execute(script) {
                            Ok(_) => {
                                print!(" Done!\n");
                                db
                            },
                            Err(e) => panic!("\nNot able to create DB:\n{}", e.to_string()),
                        }
                    },
                }
            },
            true => {
                print!("Restoring previous version...");
                match sqlite::open(file) {
                    Ok(db) => {
                        print!(" Done!\n");
                        db
                    }
                    Err(e) => panic!("\nNot able to restore the session:\n{}", e.to_string()),
                }
            }
        }
    }
}
