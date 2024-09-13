use std::fmt::{self, Formatter, Write};

/// Utilities for slices.
pub mod slice;

pub fn display_bytes(bytes: &[u8], f: &mut Formatter) -> fmt::Result {
    for chunk in bytes.utf8_chunks() {
        f.write_str(chunk.valid())?;
        f.write_str(if chunk.invalid().is_empty() {
            ""
        } else {
            "\u{FFFD}"
        })?;
    }

    Ok(())
}

pub fn debug_bytes(bytes: &[u8], f: &mut Formatter) -> fmt::Result {
    for chunk in bytes.utf8_chunks() {
        fmt::Display::fmt(&chunk.valid().escape_debug(), f)?;

        for byte in chunk.invalid() {
            write!(f, "\\x{byte:02X}")?;
        }
    }

    Ok(())
}

pub trait BytesExt {
    #[must_use]
    fn byte_str(&self) -> ByteStr<'_>;
}

impl BytesExt for [u8] {
    #[inline]
    fn byte_str(&self) -> ByteStr<'_> {
        ByteStr(self)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ByteStr<'a>(pub &'a [u8]);

impl fmt::Debug for ByteStr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_char('"')?;
        debug_bytes(self.0, f)?;
        f.write_char('"')?;

        Ok(())
    }
}

impl fmt::Display for ByteStr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        display_bytes(self.0, f)
    }
}
