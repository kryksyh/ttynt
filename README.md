# ttynt

`ttynt` is a command-line tool for coloring and highlighting text in the terminal based on regex patterns. It might be useful for log file analysis, text parsing, and more.

## Installation

To build `ttynt`, you need to have Rust installed. If you don't have Rust installed, you can install it from [rust-lang.org](https://www.rust-lang.org/).

1. Clone the repository:
    ```
    git clone https://github.com/kryksyh/ttynt.git
    cd ttynt
    ```

2. Build the project:
    ```
    cargo build --release
    ```

3. The compiled binary will be located in the `target/release` directory:
    ```
    ./target/release/ttynt
    ```

## Usage

### Basic Usage

```
ttynt [OPTIONS] <PATTERNS>...
```

### Options

- `-c, --color-whole-line`: Color the whole line instead of just the matched part.
- `-s, --case-sensitive`: Enable case-sensitive matching.

### Examples

#### Highlighting Matches

Highlight lines or parts of lines that match the regex patterns `error` and `warning`:

```
cat yourfile.log | ttynt "error" "warning"
```

#### Case-sensitive Matching

By default ttynt uses case-insensitive matching. To enable
case sensitivity use `-s` key.

Highlight matches in a case-sensitive manner:

```
cat yourfile.log | ttynt -s "Error" "Warning"
```

#### Coloring the Whole Line
By default ttynt colors only the matched part of the line. 
To color the whole line use `-c` key.

Color the entire line where a match is found:

```
cat yourfile.log | ttynt -c "error" "warning"
```

## Development

### Running Tests

To run the tests, use the following command:

```
cargo test
```
