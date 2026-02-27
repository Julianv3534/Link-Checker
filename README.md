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

## Usage

```bash
TODO
```

- `input.md`: source Markdown file (required)
- `output.md`: destination file (optional, defaults to `output.md`)

## Example

TODO
