# posix-portable-filename
A validated type for POSIX portable filenames. It uses the [newtype idiom].

[newtype idiom]: https://doc.rust-lang.org/rust-by-example/generics/new_types.html


## The Problem
Unix filesystems technically allow almost any byte sequence as a filename. This permissiveness often causes problems:

- `*`, `?`, `[`, `]` — shell glob characters
- `-rf` — interpreted as command-line flags
- Spaces, quotes, backticks — shell escaping nightmares
- Non-UTF-8 bytes — encoding hell

Most other Rust libraries *sanitize* filenames by transforming bad characters into valid ones. This crate takes a different; it only allows construction of its core type (`PortableFilename`) for a POSIX portable filename.


## Usage
```rust
use posix_portable_filename::{PortableFilename, InvalidFilename};

fn main() -> Result<(), InvalidFilename> {
    let name = PortableFilename::new("my_file.txt")?;
    // `new()` only succeeds if the filename conforms.
    
    println!("{}", name); // Display impl
    let s: &str = name.as_str(); // Deref to &str

    // Invalid inputs are rejected
    assert!(PortableFilename::new("foo/bar").is_err()); // path separator
    assert!(PortableFilename::new("-rf").is_err());     // leading hyphen
    assert!(PortableFilename::new("foo bar").is_err()); // space
    assert!(PortableFilename::new("..").is_err());      // reserved
    Ok(())
}
```


## Design Philosophy
**Parse, don't validate.** Once you have a `PortableFilename`, you know it's valid. No need to re-check at every use site. The type system enforces the invariant.


## The Standard
POSIX.1-2008 defines the **Portable Filename Character Set**:
```txt
A B C D E F G H I J K L M N O P Q R S T U V W X Y Z
a b c d e f g h i j k l m n o p q r s t u v w x y z
0 1 2 3 4 5 6 7 8 9 . _ -
```
Additionally:
- Filenames cannot start with `-` since this risks confusion with command-line options.
- `.` and `..` are reserved for directory traversal
- Maximum 255 bytes (common filesystem limit)


## Features
This crate provides one optional [feature].

### `serde`
Serialize/deserialize with validation on deserialization
```toml
# Cargo.toml
[dependencies]
posix-portable-filename = { version = "0.1", features = ["serde"] }
```

[feature]: https://doc.rust-lang.org/cargo/reference/features.html


## Correctness
This library contains no unsafe code (`#![forbid(unsafe_code)]`) and has a small, auditable validation path.

Testing includes:
- _Unit tests_ covering valid inputs, all rejection cases, boundary conditions (empty, max length), and trait implementations
- _Fuzz testing*_ with over 1 billion iterations to coverage saturation, with no crashes or panics

The validation logic is ~30 lines of straightforward character checking; see `PortableFilename::new()`.

The fuzz harness feeds arbitrary byte sequences to the parser; coverage saturation means all reachable code paths have been exercised.


## Fuzz Testing
After you install [cargo fuzz] as recommended (which involves using Nightly Rust), then you can run fuzz testing with:
```sh
cargo +nightly fuzz run fuzz_new
```
Note: "rwsv" stands for "read, write, seek, validate" -- the methods exercised by the fuzzer.
[cargo fuzz]: https://github.com/rust-fuzz/cargo-fuzz


## License
Dual-licensed under MIT or Apache 2.0, at your option.


## References
- [POSIX.1-2008 §3.282 — Portable Filename Character Set](https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap03.html#tag_03_282)
- [POSIX.1-2008 §4.7 — Filename Portability](https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap04.html#tag_04_07)
- [David Wheeler — Fixing Unix/Linux/POSIX Filenames](https://dwheeler.com/essays/fixing-unix-linux-filenames.html)
