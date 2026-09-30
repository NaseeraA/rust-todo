# rust-todo

A small command-line todo list, built as a project for learning Rust.

## Usage

Requires [Rust](https://www.rust-lang.org/tools/install).

```sh
cargo run -- list              # show all todos
cargo run -- add "buy milk"    # add a todo (quote items with spaces)
cargo run -- done 2            # mark todo number 2 as complete
```

Example output:

```
1. [ ] Bread
2. [x] Bananas
```

## Status

Work in progress. Todos are held in memory for now, so changes are lost when the program exits. Saving to a file is next.
