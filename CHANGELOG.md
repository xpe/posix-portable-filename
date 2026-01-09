# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] - 2026-01-09

### Changed

- edited the README only

## [0.1.0] - 2025-01-08

### Added

- Initial release
- `PortableFilename` newtype with validation
- POSIX portable filename character set enforcement (`A-Za-z0-9._-`)
- Rejection of leading hyphens, `.`, `..`, and filenames exceeding 255 bytes
- `FromStr`, `TryFrom<String>`, `TryFrom<&str>` implementations
- `AsRef<str>`, `AsRef<Path>`, `Deref<Target=str>` implementations
- `Display`, `Debug`, `Clone`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`
- Optional `serde` feature for serialize/deserialize with validation
