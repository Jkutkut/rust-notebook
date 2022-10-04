use sqlx::{sqlite::SqliteQueryResult, Sqlite, SqlitePool, migrate::MigrateDatabase};

pub struct NotebookDB {
    pub file: String,
}

impl NotebookDB {
    pub fn new(file: &str) -> Self {
        let db: NotebookDB = NotebookDB {
            file: String::from(file),
        };
        init_db(&db);
        db
    }

    pub fn list(&self, category: &String) {
        // TODO
        print!("Listing by {category}");
    }

    pub fn add(&self, category: &String) {
        // TODO
        print!("Adding a {category}\n");
    }

    pub fn remove(&self, category: &String) {
        // TODO
        print!("Removing {category}\n");
    }
}

// Functions:

fn init_db(db: &NotebookDB) {
    print!("Attempting to restore DB...\n");
    print!("  File: {}", db.file);
    // TODO create DB.
    // TODO recover DB if exist
}
