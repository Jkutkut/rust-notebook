# Rust notebook:

A simple, command used, notebook/agenda program to keep all your notes organized.

## Usage:
- ```list```: Show all the elements.
- ```add```: Opens a form to fill all the data.
- ```remove```: Removes a element by name.
- ```help```: Show the usage menu.
- ```clear```: Clear the terminal.
- ```exit```: Exit the program.

## Compilation:

The compilation of the program is done with cargo.

```
cargo build --release
```

Executable: ```target/release/rust-notebook```

You can also use ```cargo run``` to build and run directly the program.


## TODO
- [ ] v1.1.0 - Define new data logic to have categories and subcategories.
	- With a simple SQliteDB
- [ ] v1.2.0 - Define new command logic.
- [ ] v1.3.0 - Implement commands.
- [ ] v1.4.0 - Implement data logic.
	- [ ] Data logic.
	- [ ] Export json.
	- [ ] Inport json.
- [ ] v1.5.0 - Integrate commands with data logic.
- [ ] v1.6.0 - Multiline description.
- [ ] v1.7.0 - Refactorization and remove TODOs.
- [ ] v1.8.0 - Clear the screen on init.
- [ ] v2.0.0 - Documentation and final version.
