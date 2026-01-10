use std::fmt;
use std::str::FromStr;
use thiserror::Error;

/// Error returned when a string is not a valid POSIX portable filename.
#[derive(Debug, Clone, Error)]
#[error("invalid portable filename: {reason}")]
pub struct InvalidFilename {
    reason: &'static str,
}

impl InvalidFilename {
    /// Returns the reason the filename was rejected.
    #[must_use]
    pub fn reason(&self) -> &'static str {
        self.reason
    }
}

/// A validated POSIX portable filename.
///
/// Guaranteed to be:
/// - Non-empty
/// - Not `.` or `..`
/// - Composed only of `A-Za-z0-9._-`
/// - Not starting with `-`
///
/// Construct via [`PortableFilename::new`], [`FromStr`], or [`TryFrom`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PortableFilename(String);

impl PortableFilename {
    /// Maximum allowed length in **bytes** (POSIX filesystem limit).
    pub const MAX_LEN: usize = 255;

    /// Validate and construct a new portable filename.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidFilename`] if the input:
    /// - Is empty
    /// - Is `.` or `..`
    /// - Contains characters outside `A-Za-z0-9._-`
    /// - Starts with `-`
    /// - Exceeds 255 bytes
    pub fn new(s: impl Into<String>) -> Result<Self, InvalidFilename> {
        let s = s.into();

        if s.is_empty() {
            return Err(InvalidFilename {
                reason: "cannot be empty",
            });
        }

        if s.len() > Self::MAX_LEN {
            return Err(InvalidFilename {
                reason: "exceeds 255 bytes",
            });
        }

        if s == "." || s == ".." {
            return Err(InvalidFilename {
                reason: "cannot be '.' or '..'",
            });
        }

        // Validate each character, handling the leading hyphen rule specially.
        for (i, c) in s.chars().enumerate() {
            if i == 0 && c == '-' {
                return Err(InvalidFilename {
                    reason: "cannot start with '-'",
                });
            }
            if !is_portable(c) {
                return Err(InvalidFilename {
                    reason: "contains non-portable characters",
                });
            }
        }

        Ok(PortableFilename(s))
    }

    /// Returns the filename as a string slice.
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes self and returns the inner `String`.
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}

/// Returns `true` if `c` is an allowed character for a POSIX portable filename.
///
/// Note: hyphen (`-`) is permitted here, but a separate check ensures it is not the first character.
#[inline]
fn is_portable(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '-'
}

impl AsRef<str> for PortableFilename {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsRef<std::path::Path> for PortableFilename {
    #[inline]
    fn as_ref(&self) -> &std::path::Path {
        std::path::Path::new(&self.0)
    }
}

impl std::ops::Deref for PortableFilename {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::borrow::Borrow<str> for PortableFilename {
    #[inline]
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PortableFilename {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Implements parsing from a string slice via the `FromStr` trait.
impl FromStr for PortableFilename {
    type Err = InvalidFilename;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

/// Implements conversion from a `String` via the `TryFrom` trait.
impl TryFrom<String> for PortableFilename {
    type Error = InvalidFilename;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

/// Implements conversion from a string slice via the `TryFrom` trait.
impl<'a> TryFrom<&'a str> for PortableFilename {
    type Error = InvalidFilename;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

impl From<PortableFilename> for String {
    fn from(name: PortableFilename) -> Self {
        name.0
    }
}

impl From<PortableFilename> for std::path::PathBuf {
    fn from(name: PortableFilename) -> Self {
        std::path::PathBuf::from(name.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_filenames() {
        let cases = [
            "foo",
            "foo_bar",
            "foo-bar",
            "foo.bar",
            "FooBar123",
            "_leading_underscore",
            ".hidden",
            "a",
            "...",
            "0",
            "123",
            "a.b.c.d",
        ];
        for case in cases {
            assert!(
                PortableFilename::new(case).is_ok(),
                "should accept: {}",
                case
            );
        }
    }

    #[test]
    fn empty_rejected() {
        let err = PortableFilename::new("").unwrap_err();
        assert_eq!(err.reason(), "cannot be empty");
    }

    #[test]
    fn dot_dotdot_rejected() {
        assert!(PortableFilename::new(".").is_err());
        assert!(PortableFilename::new("..").is_err());
    }

    #[test]
    fn leading_hyphen_rejected() {
        let cases = ["-foo", "-", "--flag", "-rf"];
        for case in cases {
            let err = PortableFilename::new(case).unwrap_err();
            assert_eq!(err.reason(), "cannot start with '-'", "for input: {}", case);
        }
    }

    #[test]
    fn non_portable_chars_rejected() {
        let cases = [
            ("foo/bar", "path separator"),
            ("foo bar", "space"),
            ("foo*", "asterisk"),
            ("foo?bar", "question mark"),
            ("foo:bar", "colon"),
            ("foo\0bar", "null"),
            ("café", "non-ASCII"),
            ("foo\nbar", "newline"),
            ("foo\\bar", "backslash"),
            ("foo<bar", "less than"),
            ("foo>bar", "greater than"),
            ("foo|bar", "pipe"),
            ("foo\"bar", "quote"),
            ("foo'bar", "apostrophe"),
            ("foo`bar", "backtick"),
            ("foo!bar", "exclamation"),
            ("foo@bar", "at sign"),
            ("foo#bar", "hash"),
            ("foo$bar", "dollar"),
            ("foo%bar", "percent"),
            ("foo^bar", "caret"),
            ("foo&bar", "ampersand"),
            ("foo(bar", "open paren"),
            ("foo)bar", "close paren"),
            ("foo=bar", "equals"),
            ("foo+bar", "plus"),
            ("foo[bar", "open bracket"),
            ("foo]bar", "close bracket"),
            ("foo{bar", "open brace"),
            ("foo}bar", "close brace"),
            ("foo;bar", "semicolon"),
            ("foo,bar", "comma"),
            ("foo~bar", "tilde"),
        ];
        for (case, desc) in cases {
            assert!(
                PortableFilename::new(case).is_err(),
                "should reject {} ({})",
                case,
                desc
            );
        }
    }

    #[test]
    fn max_length_enforced() {
        let long = "a".repeat(255);
        assert!(PortableFilename::new(&long).is_ok());

        let too_long = "a".repeat(256);
        let err = PortableFilename::new(&too_long).unwrap_err();
        assert_eq!(err.reason(), "exceeds 255 bytes");
    }

    #[test]
    fn error_display() {
        let err = PortableFilename::new("").unwrap_err();
        assert_eq!(
            err.to_string(),
            "invalid portable filename: cannot be empty"
        );
    }

    #[test]
    fn from_str_works() {
        let name: PortableFilename = "test.txt".parse().unwrap();
        assert_eq!(name.as_str(), "test.txt");
    }

    #[test]
    fn try_from_works() {
        let name = PortableFilename::try_from("test.txt").unwrap();
        assert_eq!(name.as_str(), "test.txt");

        let name = PortableFilename::try_from(String::from("test.txt")).unwrap();
        assert_eq!(name.as_str(), "test.txt");
    }

    #[test]
    fn into_string_works() {
        let name = PortableFilename::new("test.txt").unwrap();
        let s: String = name.into();
        assert_eq!(s, "test.txt");
    }

    #[test]
    fn as_path_works() {
        let name = PortableFilename::new("test.txt").unwrap();
        let path: &std::path::Path = name.as_ref();
        assert_eq!(path, std::path::Path::new("test.txt"));
    }

    #[test]
    fn deref_works() {
        let name = PortableFilename::new("test.txt").unwrap();
        assert!(name.ends_with(".txt"));
        assert_eq!(name.len(), 8);
    }

    #[test]
    fn ordering_works() {
        let a = PortableFilename::new("aaa").unwrap();
        let b = PortableFilename::new("bbb").unwrap();
        assert!(a < b);
    }

    #[test]
    fn borrow_enables_hashmap_lookup() {
        use std::collections::HashMap;

        let mut map: HashMap<PortableFilename, i32> = HashMap::new();
        let key = PortableFilename::new("config.txt").unwrap();
        map.insert(key, 42);

        // Look up using &str via Borrow<str>
        assert_eq!(map.get("config.txt"), Some(&42));
    }
}
