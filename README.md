# Rust notebook:

A simple, command used, notebook/agenda program to keep all your notes organized.

## Usage:
- ```help```: Show the usage menu.
- ```clear```: Clear the terminal.
- ```exit```: Exit the program.
- ```list```: Shows all the notes.
	- ```list categories```: Lists all categories.
	- ```list tags```: Lists all categories.
	- ```list category```: Shows the notes in all categories.
	- ```list category <CATEGORY>```: Shows the notes in a category.
	- ```list tag```: Shows the notes in all tags.
	- ```list tag <TAG>```: Shows the notes with a tag.
- ```add```: Adds notes to DB.
	- ```add note```: Adds a new note.
	- ```add category XXXX```: Adds a new category called XXXX.
	- ```add category```: Adds a new category.
	- ```add tag XXXX```: Adds a new tag called XXXX.
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

Keep in mind you need to specify the location of the SQLite DB file in the first parameter.

### Docker image:
In order to execute the program without installing Rust, you can use the Docker image.

The docker file used to build the image is in the root of the project. However, you can use the image directly from Docker Hub.

```
docker pull jkutkut/rust-notebook
```

In order for the program to work, a location to store the SQLite DB file must be specified. The next command will create a Docker Volume to store the DB file when executing the container.

```
docker run -v rust-notebook-db:/db -i --rm jkutkut/rust-notebook
```

Note: The implementation of the Docker image is just a proof of concept. The intended use is compiling the program.


## Version history:
- [x] v1.1.0 - Define new data logic to have categories and subcategories.
	- With a simple SQliteDB
- [x] v1.2.0 - Define new command logic.
- [x] v1.3.0 - Implement commands to work with DB.
- [x] v1.4.0 - Implement data logic.
	- [x] Data logic.
	- [x] Export data.
	- [x] Import data.
- [x] v1.5.0 - Refactor with sqlite crate.
	- [x] Refactor file and file_url.
	- [x] Be able to restore session again.
	- [x] Create DB script.
- [x] v1.6.0 - Integrate commands with data logic.
	- [x] Integration with DB.
	- [x] Error handling.
- [x] v1.7.0 - Refactorization and remove TODOs.
	- [x] Help menu.
	- [x] TODOs.
- [x] v1.8.0 - Multiline description.
- [x] v1.9.0 - Title and clear the screen on init.
- [x] v2.0.0 - Documentation and final version.
- [x] v2.1.0 - First Docker image.
- [x] v2.2.0 - Docker image integration with dockerhub.
