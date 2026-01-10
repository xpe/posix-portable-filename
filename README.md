# posix-portable-filename

A validated type for POSIX portable filenames. It uses the [newtype idiom].

[newtype idiom]: https://doc.rust-lang.org/rust-by-example/generics/new_types.html

## The Problem

Unix filesystems technically allow almost any byte sequence as a filename. David Wheeler [explains the situation well](https://dwheeler.com/essays/fixing-unix-linux-filenames.html):

> This lack of limitations is flexible, but it also creates a legion of unnecessary problems. In particular, this lack of limitations makes it unnecessarily difficult to write correct programs (enabling many security flaws). It also makes it impossible to consistently and accurately display filenames, causes portability problems, and confuses users.

Some examples of what can go wrong:

- A filename like `-rf` gets interpreted as command-line flags
- Spaces, quotes, and backticks create shell escaping nightmares
- Control characters (including newline and tab) are permitted
- No encoding is enforced, so filenames may not be valid UTF-8
- Characters like `*`, `?`, `[`, `]` double as shell glob metacharacters

Most other Rust libraries _sanitize_ filenames by transforming bad characters into valid ones. This crate takes a different approach; it only allows construction of its core type (`PortableFilename`) for a POSIX portable filename.

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

_Parse, don't validate._ Once you have a `PortableFilename`, you know it's valid. No need to re-check at every use site. The type system enforces the invariant.

## The Standard

POSIX.1-2008 defines the _Portable Filename Character Set_:

```text
A B C D E F G H I J K L M N O P Q R S T U V W X Y Z
a b c d e f g h i j k l m n o p q r s t u v w x y z
0 1 2 3 4 5 6 7 8 9 . _ -
```

Additionally:

- Filenames cannot start with `-` since this risks confusion with command-line options.
- `.` and `..` are reserved for directory traversal
- Maximum 255 bytes (common filesystem limit)

## Features

This crate provides two optional [features]: `arbitrary` and `serde`.

[features]: https://doc.rust-lang.org/cargo/reference/features.html

### `arbitrary`

Implements [`Arbitrary`](https://docs.rs/arbitrary) for `PortableFilename`, enabling structure-aware fuzzing with [`cargo-fuzz`](https://github.com/rust-fuzz/cargo-fuzz). Downstream crates can fuzz code that handles filenames without wasting cycles on invalid inputs.

```toml
# Cargo.toml
[dependencies]
posix-portable-filename = { version = "0.2", features = ["arbitrary"] }
```

```rust,ignore
// fuzz/fuzz_targets/my_target.rs
libfuzzer_sys::fuzz_target!(|filename: PortableFilename| {
    my_function_that_takes_filename(filename);
});
```

### `serde`

Serialize/deserialize with validation on deserialization

```toml
# Cargo.toml
[dependencies]
posix-portable-filename = { version = "0.2", features = ["serde"] }
```

## Running Unit Tests

Since this crates has optional features, use:

```sh
cargo test --all-features
```

## Correctness

This library contains no unsafe code (`#![forbid(unsafe_code)]`) and has a small, auditable validation path. The validation logic is ~30 lines of straightforward character checking; see `PortableFilename::new()`.

Testing includes:

- _Unit tests_ covering valid inputs, all rejection cases, boundary conditions (empty, max length), and trait implementations
- _Fuzz testing\*_ with over 1 billion iterations to coverage saturation, with no crashes or panics

\* The fuzz harness feeds arbitrary byte sequences to the parser; coverage saturation means all reachable code paths have been exercised.

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
- [POSIX.1-2008 §4.8 — Filename Portability](https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap04.html#tag_04_08)
- [David Wheeler — Fixing Unix/Linux/POSIX Filenames](https://dwheeler.com/essays/fixing-unix-linux-filenames.html)
