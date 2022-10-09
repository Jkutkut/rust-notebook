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
            "categories" | "tags" => return self.list_table(table_type),
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

    pub fn list_table(&self, table_type: &str) -> Result<String, String> {
        let query;
        match table_type {
            "categories" => {
                query = "SELECT ID, CAT_NAME FROM CATEGORY ORDER BY CAT_NAME";
            },
            "tags" => {
                query = "SELECT ID, TAG_NAME FROM TAG ORDER BY TAG_NAME";
            },
            _ => return Err(String::from("Use tags or categories")),
        }
        let mut result = format!("All {table_type}:\n");
        let mut statement = self.db.prepare(query).unwrap();
        while let Ok(sqlite::State::Row) = statement.next() {
            result += format!(
                "- {} ({})\n",
                statement.read::<String>(1).unwrap(),
                statement.read::<i64>(0).unwrap()
            ).as_str();
        }
        Ok(result)
    }

    // Add

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

    fn get_id(&self, query: &str, f: &str) -> i64 {
        let mut statement = self.db.prepare(query).unwrap().bind(1, f).unwrap();
        statement.next().unwrap();
        statement.read::<i64>(0).unwrap()
    }

    pub fn add_note(&self, name: &str, description: &str,
                    category: &str, tag: &str) -> Result<String, String> {
        let cat_id = self.get_id("SELECT ID FROM CATEGORY WHERE CAT_NAME = ?;", category);
        if cat_id == 0 {
            return Err(String::from("Category not found."));
        }
        let tag_id = self.get_id("SELECT ID FROM TAG WHERE TAG_NAME = ?;", tag);
        if tag_id == 0 {
            return Err(String::from("Tag not found."));
        }
        let mut s = self.db.prepare("
                INSERT INTO NOTE (NOTE_NAME, NOTE_DESC, CATEGORY_ID, TAG_ID)
                VALUES (?, ?, ?, ?);
            ").unwrap()
            .bind(1, name).unwrap()
            .bind(2, description).unwrap()
            .bind(3, cat_id).unwrap()
            .bind(4, tag_id).unwrap();
        match s.next() {
            Ok(_) => Ok(String::from("Note added.\n")),
            Err(e) => {
                Err(e.to_string())
            },
        }
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
