# link_checker

Small Rust CLI that scans a Markdown file, checks each URL, and writes a Markdown report.

## What it does
- Extracts links from common Markdown URL formats.
- Requests each URL and reads the page `<title>`.
- Writes one line per URL to an output file:
  - Success: page title
  - Failure: error message (timeout, connection error, HTTP status, etc.)

Output line format:

```md
[ TITLE_OR_ERROR ] ( https://example.com )
```

## Requirements
- Rust toolchain (Cargo)

## Usage
```bash
cargo run -- <input.md> [output.md]
```

- `input.md`: source Markdown file (required)
- `output.md`: destination file (optional, defaults to `output.md`)

## Example
Input markdown:

```md
- [Example](https://example.com)
- <https://rust-lang.org>
- https://crates.io
```

Run:

```bash
cargo run -- links.md checked_links.md
```

## Makefile shortcuts
```bash
make test
make build
make run ARGS="-- links.md checked_links.md"
make clean
```
