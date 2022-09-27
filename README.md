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



## Elements:
- Name
- Description: optional
- [TODO] Category
- [TODO] Subcategory: value o general.

## Future features:
- Refactorization.
- Categories to keep elements categorized.
- Subcategories in each category.

- ```category```:
	- ```add```
	- ```remove```
- Remove category.
- Remove subcategory.
- Multiline description
