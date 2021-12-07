# 2048

This is an implementation of the game `2048` in the Rust programming language
for the testing and development of an expectimax AI solver.

```
[2048]                   Score:     69872
-----------------------------------------
|  4096   |  2048   |   512   |   16    |
-----------------------------------------
|   16    |   64    |   128   |   32    |
-----------------------------------------
|    8    |   32    |   16    |    8    |
-----------------------------------------
|    2    |    4    |    2    |    4    |
-----------------------------------------
```

## table of contents
* [usage](#usage)
* [license](#license)

## usage

To run the game with the expectimax solver:
```
cargo run -- --solver expectimax
```

Options:
```
USAGE:
    2048 [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -s, --solver <solver>    Selects the solver to use
```

## license

[LICENSE](LICENSE)
