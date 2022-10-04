// extern crate serde;
// extern crate serde_json;
extern crate sqlx;

mod notebook;
use notebook::Notebook;

mod shell;

mod notebook_sqlite;

#[async_std::main]
async fn main() {
    let mut nb: Notebook = Notebook::new("sqlite://notebook.db").await;
    nb.run();
}
