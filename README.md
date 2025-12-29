# aegrep

`aegrep` is a small Rust command-line tool inspired by `grep` and the Rust Book's
*minigrep* project. It searches for a pattern in one or more files and prints matching
lines with colored, nicely formatted output.

This project is mainly a learning playground for:

- Error handling with custom enums and `Result`
- Module organization with `main.rs`, `lib.rs`, and multiple submodules
- Basic CLI argument parsing
- Simple text searching and line highlighting

## Features

- Search for a **pattern** across one or more files
- Show matches as `file:line:col line_content`
- Highlight the matched pattern using terminal colors (via `owo-colors`)
- Custom error type (`MyErrors`) for argument and I/O errors
- Config struct with a simple builder-style method (`with_quiet`)

## Usage

From the project root:

```bash
cargo run -- <pattern> <file> [more_files...]
```

Examples:

```bash
# Search for "fox" in poem.txt
cargo run -- fox poem.txt

# Search in multiple files
cargo run -- error src/main.rs src/lib.rs
```

If required arguments are missing, the program exits with a descriptive panic message.

## Project Structure

```plaintext
src/
  main.rs    # entry point, calls CLI + search
  lib.rs     # library: search logic, single_search, etc.
  cli.rs     # parse_args(): builds Config, returns Result<Config, MyErrors>
  types.rs   # MyErrors enum + Config struct
  pretty.rs  # print_found(): colored, formatted output
```

- `main.rs`
  - Calls `cli::parse_args()` to get a `Config`
  - On success, calls `aegrep::search(...)`
- `lib.rs`
  - Exposes `search(...)` and wires together `single_search`, `pretty`, and errors
- `types.rs`
  - `MyErrors`: `FileReadError(String)`, `MissingArgsError`, etc.
  - `Config`: `{ pattern, files, quiet }` with `Config::new` and `with_quiet`
- `pretty.rs`
  - Uses `owo-colors` to highlight the pattern and format the output line

## Building and Running

```bash
# Build
cargo build

# Run with arguments
cargo run -- pattern file1 [file2 ...]
```

Run with `--release` for an optimized binary:

```bash
cargo build --release
```

## Future Ideas

- Add a `--quiet` or `-q` flag that hooks into `Config::with_quiet`
- Add case-insensitive search (e.g. `--ignore-case`)
- Add `--line-number` / `--column` toggles
- Add tests for `parse_args`, `search`, and matching logic

This project is primarily for learning Rust patterns around CLI tools, error handling,
and module organization.
