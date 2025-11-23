//! Contains the `StringRange` struct for identifying substrings.
//!
//! String ranges use zero-based Unicode character offsets to identify portions of text, commonly
//! used in autocomplete results to highlight matching segments.

use std::ops::Range;

// -------------------------------------------------------------------------------------------------
//
/// Identifies a substring within a given text using character offsets.
///
/// Uses zero-based Unicode character offsets where `start_offset` is inclusive and `end_offset` is
/// exclusive.
///
/// Commonly used in autocomplete responses to indicate which portions of text matched the user's
/// input, allowing UI highlighting of relevant segments.
///
/// Note: Offsets are Unicode character positions, not byte positions, which is important for proper
/// handling of multi-byte characters.
#[derive(
    // std
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    // serde
    serde::Deserialize,
    serde::Serialize,
    // getset
    getset::Getters,
    getset::CopyGetters,
    getset::MutGetters,
    getset::Setters,
    // other
    bon::Builder
)]
#[serde(rename_all = "camelCase")]
pub struct StringRange {
    /// Zero-based offset of the first Unicode character (inclusive).
    ///
    /// The starting position of the substring within the parent text.
    #[serde(default)]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub start_offset: u32,

    /// Zero-based offset of the last Unicode character (exclusive).
    ///
    /// The ending position of the substring, where the character at this position is not included
    /// in the range.
    #[serde(default)]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub end_offset: u32,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl StringRange {
    /// Creates a new `StringRange` from start and end offsets.
    ///
    /// Creates a range where `start_offset` is inclusive and `end_offset` is exclusive, matching
    /// standard range semantics.
    ///
    /// Use this when constructing ranges from API responses or when manually specifying text
    /// segments to highlight.
    #[must_use]
    pub const fn new(start_offset: u32, end_offset: u32) -> Self {
        Self {
            start_offset,
            end_offset,
        }
    }

    /// Returns the start offset (inclusive).
    ///
    /// The zero-based Unicode character position where the range begins.
    #[must_use]
    pub const fn start(self) -> u32 {
        self.start_offset
    }

    /// Returns the end offset (exclusive).
    ///
    /// The zero-based Unicode character position where the range ends. The character at this
    /// position is not included in the range.
    #[must_use]
    pub const fn end(self) -> u32 {
        self.end_offset
    }

    /// Returns the length of the range in characters.
    ///
    /// Calculates the number of Unicode characters covered by this range.
    ///
    /// Use this to determine how many characters to highlight or extract.
    #[must_use]
    pub const fn len(self) -> u32 {
        self.end_offset - self.start_offset
    }

    /// Checks if the range is empty (zero length).
    ///
    /// Returns `true` if the start and end offsets are equal, indicating a zero-length range that
    /// covers no characters.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.start_offset == self.end_offset
    }

    /// Converts to a Rust standard `Range<usize>`.
    ///
    /// Creates a `Range<usize>` for use with string slicing operations.
    ///
    /// Use this when you need to extract or highlight the substring using Rust's standard range
    /// syntax. Returns `None` if either offset is negative.
    #[must_use]
    pub const fn range(self) -> Range<usize> {
        (self.start_offset as usize)..(self.end_offset as usize)
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl From<Range<usize>> for StringRange {
    /// Converts a Rust `Range<usize>` to a `StringRange`.
    ///
    /// Creates a `StringRange` from a standard Rust range.
    ///
    /// Use this when constructing API requests from range-based string operations or when you have
    /// computed offsets using Rust's range syntax.
    fn from(range: Range<usize>) -> Self {
        Self {
            start_offset: u32::try_from(range.start).unwrap_or_default(),
            end_offset: u32::try_from(range.end).unwrap_or_default(),
        }
    }
}

impl From<&Range<usize>> for StringRange {
    /// Converts a borrowed `&Range<usize>` to a `StringRange`.
    ///
    /// Creates a `StringRange` from a borrowed standard Rust range.
    ///
    /// Use this when you want to convert a range reference without consuming the original.
    fn from(range: &Range<usize>) -> Self {
        Self {
            start_offset: u32::try_from(range.start).unwrap_or_default(),
            end_offset: u32::try_from(range.end).unwrap_or_default(),
        }
    }
}

impl TryFrom<StringRange> for Range<usize> {
    type Error = std::num::TryFromIntError;

    /// Converts a `StringRange` to a Rust `Range<usize>`.
    ///
    /// Attempts to convert the `StringRange` offsets to `usize` for use with string slicing.
    ///
    /// # Errors
    ///
    /// Returns an error if either offset is negative or too large to fit in a `usize`.
    fn try_from(string_range: StringRange) -> Result<Self, Self::Error> {
        Ok(Self {
            start: string_range.start_offset.try_into()?,
            end: string_range.end_offset.try_into()?,
        })
    }
}

impl TryFrom<&StringRange> for Range<usize> {
    type Error = std::num::TryFromIntError;

    /// Converts a borrowed `&StringRange` to a Rust `Range<usize>`.
    ///
    /// Attempts to convert the `StringRange` offsets to `usize` for use with string slicing without
    /// consuming the original.
    ///
    /// # Errors
    ///
    /// Returns an error if either offset is negative or too large to fit in a `usize`.
    fn try_from(string_range: &StringRange) -> Result<Self, Self::Error> {
        Ok(Self {
            start: string_range.start_offset.try_into()?,
            end: string_range.end_offset.try_into()?,
        })
    }
}

impl std::fmt::Display for StringRange {
    /// Formats the range as `[start..end]`.
    ///
    /// Uses Rust's standard range notation for machine-readable output. Suitable for debugging,
    /// logging, and serialization to text formats.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}..{}]", self.start_offset, self.end_offset)
    }
}