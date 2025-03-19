# A PNG Encoder

This project represent a message
encoder for a PNG file. It is implemented
in Rust.

## Features of the program

- encoding messages;
- decoding messages;
- print the bytes of a PNG file;
- remove a chunk from a PNG file;

### How to run the program

Each command can be run separately. I used
clap library for parsing the commands and
their arguments. For each command is a specific
way to run it. In `arguments.rs` file are located
the structures that represent the arguments of the
commands.

For encode command:

```
cargo run encode <file_path> <chunk_type> <message>;
```

For decode command:

```
cargo run decode <file_path> <chunk_type>;
```

For remove command:

```
cargo run remove <file_path> <chunk_type>;
```

For print command:

```
cargo run print <file_path>;
```

**OBSERVATION:** The decode and remove commands
will action for the first chunk that have the type
requested by the commands.

For information about the project:

```
cargo run -- --help;
```

#### Structure of the program

`chunk_types.rs` - contains the chunk type
structure and functions from converts `[u8; 4]`
arrays and `str` to a chunk_type structure;

`chunk.rs` - representing the chunks of the
PNG file. Each chunk contains a length (4 bytes),
a type (4 bytes), data (length size) and a crc (4
bytes). The crc is used for verifying if the chunk
is still valid;

`png.rs` - representing the PNG that contains a
`STANDARD_HEADER` and a vec of chunks;

`commands.rs` - this file contains the code
for the four commands;

`arguments.rs` - contains the structures that
represent the arguments of the commands;

`main.rs` - this is the file that runs
the commands. Contains a match that
verifies the type of command.
