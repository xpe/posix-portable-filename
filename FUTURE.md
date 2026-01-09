# Future Improvements

Potential enhancements for future versions.

## `const fn` Validation

When Rust stabilizes const trait methods, `PortableFilename::new()` could become `const fn`, enabling compile-time validation of filename literals:

```rust
// Hypothetical future syntax
const CONFIG: PortableFilename = PortableFilename::new("config.txt").unwrap();
```

This would catch invalid filenames at compile time rather than runtime.

Tracking: Requires stabilization of `const_trait_impl` and related features:
- [`const_trait_impl` in the Rust Unstable Book](https://doc.rust-lang.org/beta/unstable-book/language-features/const-trait-impl.html)
- [Tracking Issue for RFC 3762, "Make trait methods callable in const contexts" #143874](https://github.com/rust-lang/rust/issues/143874)
