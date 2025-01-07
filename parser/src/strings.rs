// Adapted from pulldown-cmark, which comes with the following license:
//
// Copyright 2015 Google Inc. All rights reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

//! String types that facilitate parsing.

use std::{
    borrow::{Borrow, Cow},
    fmt,
    hash::{Hash, Hasher},
    ops::Deref,
    str::from_utf8,
};

pub(crate) const MAX_INLINE_STR_LEN: usize = 3 * std::mem::size_of::<isize>() - 2;

/// Returned when trying to convert a `&str` into an [`InlineStr`] but it fails
/// because it doesn't fit.
#[derive(Debug)]
pub struct StringTooLongError;

/// An inline string that can contain almost three words
/// of UTF-8 text.
#[derive(Debug, Clone, Copy, Eq)]
pub struct InlineStr {
    inner: [u8; MAX_INLINE_STR_LEN],
    len: u8,
}

impl AsRef<str> for InlineStr {
    fn as_ref(&self) -> &str {
        self.deref()
    }
}

impl Hash for InlineStr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.deref().hash(state);
    }
}

impl From<char> for InlineStr {
    fn from(c: char) -> Self {
        let mut inner = [0u8; MAX_INLINE_STR_LEN];
        c.encode_utf8(&mut inner);
        let len = c.len_utf8() as u8;
        Self { inner, len }
    }
}

impl std::cmp::PartialEq<InlineStr> for InlineStr {
    fn eq(&self, other: &InlineStr) -> bool {
        self.deref() == other.deref()
    }
}

impl TryFrom<&str> for InlineStr {
    type Error = StringTooLongError;

    fn try_from(s: &str) -> Result<InlineStr, StringTooLongError> {
        let len = s.len();
        if len <= MAX_INLINE_STR_LEN {
            let mut inner = [0u8; MAX_INLINE_STR_LEN];
            inner[..len].copy_from_slice(s.as_bytes());
            let len = len as u8;
            Ok(Self { inner, len })
        } else {
            Err(StringTooLongError)
        }
    }
}

impl Deref for InlineStr {
    type Target = str;

    fn deref(&self) -> &str {
        let len = self.len as usize;
        from_utf8(&self.inner[..len]).unwrap_or_default()
    }
}

impl fmt::Display for InlineStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

/// A copy-on-write string that can be owned, borrowed
/// or inlined.
///
/// It is three words long.
#[derive(Debug, Eq)]
pub enum CowStr<'a> {
    /// An owned, immutable string.
    Boxed(Box<str>),
    /// A borrowed string.
    Borrowed(&'a str),
    /// A short inline string.
    Inlined(InlineStr),
}

impl AsRef<str> for CowStr<'_> {
    fn as_ref(&self) -> &str {
        self.deref()
    }
}

impl Hash for CowStr<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.deref().hash(state);
    }
}

impl std::clone::Clone for CowStr<'_> {
    fn clone(&self) -> Self {
        match self {
            CowStr::Boxed(s) => match InlineStr::try_from(&**s) {
                Ok(inline) => CowStr::Inlined(inline),
                Err(..) => CowStr::Boxed(s.clone()),
            },
            CowStr::Borrowed(s) => CowStr::Borrowed(s),
            CowStr::Inlined(s) => CowStr::Inlined(*s),
        }
    }
}

impl<'a> std::cmp::PartialEq<CowStr<'a>> for CowStr<'a> {
    fn eq(&self, other: &CowStr<'_>) -> bool {
        self.deref() == other.deref()
    }
}

impl<'a> From<&'a str> for CowStr<'a> {
    fn from(s: &'a str) -> Self {
        CowStr::Borrowed(s)
    }
}

impl From<String> for CowStr<'_> {
    fn from(s: String) -> Self {
        CowStr::Boxed(s.into_boxed_str())
    }
}

impl From<char> for CowStr<'_> {
    fn from(c: char) -> Self {
        CowStr::Inlined(c.into())
    }
}

impl<'a> From<Cow<'a, str>> for CowStr<'a> {
    fn from(s: Cow<'a, str>) -> Self {
        match s {
            Cow::Borrowed(s) => CowStr::Borrowed(s),
            Cow::Owned(s) => CowStr::Boxed(s.into_boxed_str()),
        }
    }
}

impl<'a> From<CowStr<'a>> for Cow<'a, str> {
    fn from(s: CowStr<'a>) -> Self {
        match s {
            CowStr::Boxed(s) => Cow::Owned(s.to_string()),
            CowStr::Inlined(s) => Cow::Owned(s.to_string()),
            CowStr::Borrowed(s) => Cow::Borrowed(s),
        }
    }
}

impl<'a> From<Cow<'a, char>> for CowStr<'a> {
    fn from(s: Cow<'a, char>) -> Self {
        CowStr::Inlined(InlineStr::from(*s))
    }
}

impl Deref for CowStr<'_> {
    type Target = str;

    fn deref(&self) -> &str {
        match self {
            CowStr::Boxed(ref b) => b,
            CowStr::Borrowed(b) => b,
            CowStr::Inlined(ref s) => s.deref(),
        }
    }
}

impl Borrow<str> for CowStr<'_> {
    fn borrow(&self) -> &str {
        self.deref()
    }
}

impl CowStr<'_> {
    /// Convert the `CowStr` into an owned `String`.
    pub fn into_string(self) -> String {
        match self {
            CowStr::Boxed(b) => b.into(),
            CowStr::Borrowed(b) => b.to_owned(),
            CowStr::Inlined(s) => s.deref().to_owned(),
        }
    }
}

impl fmt::Display for CowStr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
