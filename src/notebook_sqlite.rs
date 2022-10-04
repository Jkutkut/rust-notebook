use sqlx::{sqlite::SqliteQueryResult, Sqlite, SqlitePool, migrate::MigrateDatabase};

pub struct NotebookDB {
    pub file: String,
}

impl NotebookDB {
    pub async fn new(file: &str) -> Self {
        print!("creating a new notebookDB\n");
        let db: NotebookDB = NotebookDB {
            file: String::from(file),
        };
        init_db(&db).await;
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

async fn init_db(db: &NotebookDB) {
    print!("Attempting to restore DB...\n");
    print!("  File: {}\n", db.file);
    if !Sqlite::database_exists(&db.file).await.unwrap_or(false) {
        Sqlite::create_database(&db.file).await.unwrap();
        // TODO create DB.
    }
    // let instances = SqlitePool::connect(&db_url).await.unwrap();
    // let qry ="INSERT INTO settings (description) VALUES($1)";
    // let result = sqlx::query(&qry).bind("testing").execute(&instances).await;

    // instances.close().await;

    // println!("{:?}", result);
}
