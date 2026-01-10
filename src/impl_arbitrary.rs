//! `Arbitrary` implementation for structure-aware fuzzing.
//!
//! Generates only valid portable filenames, avoiding wasted fuzzing cycles.

use crate::PortableFilename;

use arbitrary::{Arbitrary, Result, Unstructured};

const MAX_LEN: usize = PortableFilename::MAX_LEN;

/// List of allowed characters.
///
/// The order matters; see implementation. Note that by shifting the beginning
/// or ending index inwards by one, useful subslices are formed.
const CHARS: &[u8] = b".ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_-";

const N: usize = CHARS.len();

#[cfg_attr(docsrs, doc(cfg(feature = "arbitrary")))]
impl<'a> Arbitrary<'a> for PortableFilename {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        let no_hyphen: &[u8] = &CHARS[..N - 1];
        let no_period: &[u8] = &CHARS[1..N];
        let no_hyphen_or_period: &[u8] = &CHARS[1..N - 1];

        let len: usize = u.int_in_range(1..=MAX_LEN)?;
        let mut s = String::with_capacity(len);

        if len == 1 {
            s.push(*u.choose(no_hyphen_or_period)? as char);
        } else {
            let first_char = *u.choose(no_hyphen)? as char;
            s.push(first_char);
            if len == 2 {
                if first_char == '.' {
                    s.push(*u.choose(no_period)? as char);
                } else {
                    s.push(*u.choose(CHARS)? as char);
                }
            } else {
                for _ in 2..=len {
                    s.push(*u.choose(CHARS)? as char);
                }
            }
        }

        #[cfg(test)]
        assert_eq!(len, s.len());

        Ok(PortableFilename::new(s).expect("valid filename"))
    }

    #[inline]
    fn size_hint(_depth: usize) -> (usize, Option<usize>) {
        (1, Some(MAX_LEN))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arbtest::arbtest;

    #[test]
    fn arbitrary_produces_valid_filenames() {
        arbtest(|u| {
            let _filename = PortableFilename::arbitrary(u)?;
            // If constructed, the filename is valid.
            Ok(())
        })
        .budget_ms(500);
    }
}
