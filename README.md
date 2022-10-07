# Rust notebook:

A simple, command used, notebook/agenda program to keep all your notes organized.

## Usage:
- ```help```: Show the usage menu.
- ```clear```: Clear the terminal.
- ```exit```: Exit the program.
- ```list```: Shows all the notes.
	- ```list category```: Shows the notes in all categories.
	- ```list category <CATEGORY>```: Shows the notes in a category.
	- ```list tag```: Shows the notes in all tags.
	- ```list tag <TAG>```: Shows the notes with a tag.
- ```add```: Adds notes to DB.
	- ```add note```: Adds a new note.
	- ```add category```: Adds a new category.
	- ```add tag```: Adds a new tag.
- ```remove```: Removes a element by name.
	- ```rm note```: Removes a note.
	- ```rm category```: Removes a category and the notes in that category.
	- ```rm tag```: Removes a tag and untags all the notes with this tag.

## Compilation:

The compilation of the program is done with cargo.

```
cargo build --release
```

Executable: ```target/release/rust-notebook```

You can also use ```cargo run``` to build and run directly the program.


## TODO
- [x] v1.1.0 - Define new data logic to have categories and subcategories.
	- With a simple SQliteDB
- [x] v1.2.0 - Define new command logic.
- [x] v1.3.0 - Implement commands to work with DB.
- [x] v1.4.0 - Implement data logic.
	- [x] Data logic.
	- [x] Export data.
	- [x] Import data.
- [ ] v1.5.0 - Refactor with sqlite crate.
	- [ ] Refactor file and file_url.
	- [ ] Be able to restore session again.
	- [ ] Create DB script.
- [ ] v1.6.0 - Integrate commands with data logic.
	- [ ] Integration with DB.
	- [ ] Error handling.
- [ ] v1.7.0 - Multiline description.
- [ ] v1.8.0 - Refactorization and remove TODOs.
- [ ] v1.9.0 - Clear the screen on init.
- [ ] v2.0.0 - Documentation and final version.
