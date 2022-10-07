// extern crate serde;
// extern crate serde_json;
extern crate sqlx;

mod notebook;
use notebook::Notebook;

mod shell;

mod notebook_sqlite;

use std::env;

#[async_std::main]
async fn main() {
    let argv: Vec<String> = env::args().collect();
    if argv.len() != 2 {
        if argv.len() < 2 {
            print!("Enter the DB location as first parameter\n");
        }
        else {
            print!("Too many parameters\n");
        }
        return;
    }
    let file: &str = argv[1].as_str();
    let mut nb: Notebook = Notebook::new(file).await;
    nb.run().await;
}
