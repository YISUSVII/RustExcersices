# Exercise 24 — File Reading

- **Title:** Safe File Reading
- **Objective:** Read and write files with `std::fs`, propagate I/O errors with `?`, and recover gracefully from a missing file.
- **Difficulty:** Intermediate
- **Concepts practiced:** `std::fs`, `std::io::Result`, the `?` operator, `io::ErrorKind::NotFound`, `std::env::temp_dir`

## Instructions

The program is self-contained: it creates its own file under `std::env::temp_dir()` and deletes it at the end.

1. Implement `write_sample(path: &Path, contents: &str) -> io::Result<()>`.
2. Implement `read_lines(path: &Path) -> io::Result<Vec<String>>`.
3. Implement `count_non_empty_lines(path: &Path) -> io::Result<usize>` (lines that are not just whitespace), reusing `read_lines` with `?`.
4. Implement `read_or_default(path: &Path, default: &str) -> io::Result<String>`:
   - return the file contents if it exists,
   - return `default` when the error kind is `NotFound`,
   - propagate any **other** error.
5. In `main`, write a sample file, print its lines and non-empty count, try a missing file, then clean up with `fs::remove_file`.

## Expected behavior

```text
wrote <temp dir>/rust-ex24-sample-<pid>.txt
1: first line
2:
3: third line
non-empty lines: 2
missing file -> "(no config found)"
missing file lines -> error: NotFound
cleaned up
```

## Optional hints

- `fs::write`, `fs::read_to_string`, `str::lines`.
- `match fs::read_to_string(path) { Err(e) if e.kind() == io::ErrorKind::NotFound => ..., other => other }`
- Include `std::process::id()` in the file name so parallel runs don't collide.

## Bonus challenge

Read the file line by line with `BufReader::new(File::open(path)?).lines()` instead of loading it all at once.
