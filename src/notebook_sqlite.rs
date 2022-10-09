use std::fs;

pub struct NotebookDB {
    db: sqlite::Connection
}

impl NotebookDB {
    pub fn new(file: &str) -> Self {
        NotebookDB {
            db: NotebookDB::new_db_connection(file)
        }
    }

    // List
    
    fn format_list(&self, mut statement: sqlite::Statement, title: String) -> String {
        let mut empty = true;
        let mut result = title;
        while let Ok(sqlite::State::Row) = statement.next() {
            empty = false;
            result += format!(
                "----------\n{}:     c: {} t: {}\n\n{}\n----------\n",
                statement.read::<String>(0).unwrap(), // Name
                statement.read::<String>(2).unwrap(), // Category
                statement.read::<String>(3).unwrap(), // Tag
                statement.read::<String>(1).unwrap() // Description
            ).as_str();
        }
        match empty {
            true => String::from("Nothing found.\n"),
            false => result,
        }
    }

    pub fn list_all(&self, table_type: &str) -> Result<String, String> {
        let query;
        match table_type {
            "category" => query = " 
                SELECT N.NOTE_NAME AS 'Name', N.NOTE_DESC AS 'Description',
                    C.CAT_NAME AS 'Category', T.TAG_NAME as 'Tag'
                FROM NOTE N, CATEGORY C, TAG T
                WHERE N.CATEGORY_ID == C.ID and T.ID = N.TAG_ID
                ORDER BY C.CAT_NAME",
            "tag" => query = "
                SELECT N.NOTE_NAME AS 'Name', N.NOTE_DESC AS 'Description',
                    C.CAT_NAME AS 'Category', T.TAG_NAME as 'Tag'
                FROM NOTE N, CATEGORY C, TAG T
                WHERE N.CATEGORY_ID == C.ID and T.ID = N.TAG_ID
                ORDER BY T.TAG_NAME;",
            "all" => query = "
                SELECT N.NOTE_NAME AS 'Name', N.NOTE_DESC AS 'Description',
                    C.CAT_NAME AS 'Category', T.TAG_NAME as 'Tag'
                FROM NOTE N, CATEGORY C, TAG T
                WHERE N.CATEGORY_ID == C.ID and T.ID = N.TAG_ID
                ORDER BY N.NOTE_NAME;",
            _ => return Err(String::from("Use category or tag")),
        }
        let statement = self.db.prepare(query).unwrap();
        Ok(self.format_list(statement, format!("Listing notes by {table_type}\n")))
    }

    pub fn list(&self, table_type: &str, t: &str) -> Result<String, String> {
        let query;
        match table_type {
            "category" => query = " 
                SELECT N.NOTE_NAME AS 'Name', N.NOTE_DESC AS 'Description',
                    C.CAT_NAME AS 'Category', T.TAG_NAME as 'Tag'
                FROM NOTE N, CATEGORY C, TAG T
                WHERE N.CATEGORY_ID == C.ID and T.ID = N.TAG_ID and
                UPPER(C.CAT_NAME) == UPPER(?);",
            "tag" => query = "
                SELECT N.NOTE_NAME AS 'Name', N.NOTE_DESC AS 'Description',
                    C.CAT_NAME AS 'Category', T.TAG_NAME as 'Tag'
                FROM NOTE N, CATEGORY C, TAG T
                WHERE N.CATEGORY_ID == C.ID and T.ID = N.TAG_ID and
                UPPER(T.TAG_NAME) == UPPER(?);",
            _ => return Err(String::from("Use category or tag")),
        }
        let statement = self.db.prepare(query).unwrap().bind(1, t).unwrap();
        Ok(self.format_list(statement, format!("Listing {table_type} {t}\n")))
    }

    // 
    pub fn add(&self, table_type: &String, t: &String) -> Result<String, String> {
        let query;
        match table_type.as_str() {
            "category" => query = "",
            "tag" => query = "",
            _ => return Err(String::from("Use category or tag")),
        }
        // TODO
        print!("Adding a {table_type}: {t}\n");
        print!("{query}\n");
        Err(String::from("Not fully implemented"))
    }

    pub fn add_note(&self, name: &String, description: &String,
                    category: &String, tag: &String) -> Result<String, String> {
        // TODO
        // TODO Category integration
        // TODO Tag integration
        print!("Adding note: {}\n{}\n", name, description);
        print!("{}  {}\n", category, tag);
        Err(String::from("Not implemented"))
    }

    pub fn remove(&self, category: &String) {
        // TODO
        print!("Removing {category}\n");
    }
}

// NotebookDB tools
impl NotebookDB {
    pub fn new_db_connection(file: &str) -> sqlite::Connection {
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
