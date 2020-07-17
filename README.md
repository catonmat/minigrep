# Mini Grep

A CLI tool for pattern matching files using REGEXP.

## How to use
### Case Sensitive execution:
```sh
# cargo run search_term /path/to/file
cargo run fRoG poem.txt
# => No results found.
```

### Case Insensitive execution:
```sh
# ENV cargo run search_term /path/to/file
CASE_INSENSITIVE=1 cargo run fRoG poem.txt
# => How public, like a frog
```
