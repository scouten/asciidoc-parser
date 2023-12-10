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

mod string_too_long_err {
    use crate::strings::StringTooLongError;

    #[test]
    fn impl_debug() {
        let e = StringTooLongError;
        assert_eq!(format!("{e:#?}"), "StringTooLongError");
    }
}

mod inline_str {
    use std::ops::Deref;

    use crate::strings::*;

    #[test]
    fn from_ascii() {
        let s: InlineStr = 'a'.into();
        assert_eq!("a", s.deref());
    }

    #[test]
    fn from_unicode() {
        let s: InlineStr = '🍔'.into();
        assert_eq!("🍔", s.deref());
    }

    #[test]
    fn impl_debug() {
        let s: InlineStr = 'a'.into();
        assert_eq!(
            format!("{s:#?}"),
            r#"InlineStr {
    inner: [
        97,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    len: 1,
}"#
        );
    }

    #[test]
    fn impl_hash() {
        use std::{
            collections::hash_map::DefaultHasher,
            hash::{Hash, Hasher},
        };

        let mut hasher = DefaultHasher::new();
        "🍔".hash(&mut hasher);
        let expected = hasher.finish();

        let s: InlineStr = '🍔'.into();
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        let actual = hasher.finish();

        let s: InlineStr = 'a'.into();
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        let mismatch = hasher.finish();

        assert_eq!(expected, actual);
        assert_ne!(expected, mismatch);
    }

    #[test]
    fn impl_partial_eq() {
        let s1: InlineStr = '🍔'.into();
        let s2: InlineStr = '🍔'.into();
        let s3: InlineStr = 'a'.into();

        assert_eq!(s1, s2);
        assert_ne!(s1, s3);
    }

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn max_len_atleast_four() {
        // we need 4 bytes to store a char
        assert!(MAX_INLINE_STR_LEN >= 4);
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn fits_twentytwo() {
        let s = "0123456789abcdefghijkl";
        let stack_str = InlineStr::try_from(s).unwrap();
        assert_eq!(stack_str.deref(), s);
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn doesnt_fit_twentythree() {
        let s = "0123456789abcdefghijklm";
        let _stack_str = InlineStr::try_from(s).unwrap_err();
    }
}

mod cow_str {
    use std::{borrow::Cow, ops::Deref};

    use crate::strings::*;

    #[test]
    fn size() {
        let size = std::mem::size_of::<CowStr>();
        let word_size = std::mem::size_of::<isize>();
        assert_eq!(3 * word_size, size);
    }

    #[test]
    fn char_to_string() {
        let c = '藏';
        let smort: CowStr = c.into();
        let owned: String = smort.to_string();
        let expected = "藏".to_owned();
        assert_eq!(expected, owned);
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn small_boxed_str_clones_to_stack() {
        let s = "0123456789abcde".to_owned();
        let smort: CowStr = s.into();
        let smort_clone = smort.clone();

        if let CowStr::Inlined(..) = smort_clone {
        } else {
            panic!("Expected a Inlined variant!");
        }
    }

    #[test]
    fn cow_to_cow_str() {
        let s = "some text";
        let cow = Cow::Borrowed(s);
        let actual = CowStr::from(cow);
        let expected = CowStr::Borrowed(s);
        assert_eq!(actual, expected);
        assert!(variant_eq(&actual, &expected));

        let s = "some text".to_string();
        let cow: Cow<str> = Cow::Owned(s.clone());
        let actual = CowStr::from(cow);
        let expected = CowStr::Boxed(s.into_boxed_str());
        assert_eq!(actual, expected);
        assert!(variant_eq(&actual, &expected));
    }

    #[test]
    fn cow_str_to_cow() {
        let s = "some text";
        let cow_str = CowStr::Borrowed(s);
        let actual = Cow::from(cow_str);
        let expected = Cow::Borrowed(s);
        assert_eq!(actual, expected);
        assert!(variant_eq(&actual, &expected));

        let s = "s";
        let inline_str: InlineStr = InlineStr::try_from(s).unwrap();
        let cow_str = CowStr::Inlined(inline_str);
        let actual = Cow::from(cow_str);
        let expected: Cow<str> = Cow::Owned(s.to_string());
        assert_eq!(actual, expected);
        assert!(variant_eq(&actual, &expected));

        let s = "s";
        let cow_str = CowStr::Boxed(s.to_string().into_boxed_str());
        let actual = Cow::from(cow_str);
        let expected: Cow<str> = Cow::Owned(s.to_string());
        assert_eq!(actual, expected);
        assert!(variant_eq(&actual, &expected));
    }

    #[test]
    fn cow_char_to_cow_str() {
        let c = 'c';
        let cow: Cow<char> = Cow::Owned(c);
        let actual = CowStr::from(cow);
        let expected = CowStr::Inlined(InlineStr::from(c));
        assert_eq!(actual, expected);
        assert!(variant_eq(&actual, &expected));

        let c = 'c';
        let cow: Cow<char> = Cow::Borrowed(&c);
        let actual = CowStr::from(cow);
        let expected = CowStr::Inlined(InlineStr::from(c));
        assert_eq!(actual, expected);
        assert!(variant_eq(&actual, &expected));
    }

    fn variant_eq<T>(a: &T, b: &T) -> bool {
        std::mem::discriminant(a) == std::mem::discriminant(b)
    }

    #[test]
    fn impl_debug() {
        let c = '藏';
        let s: CowStr = c.into();

        assert_eq!(
            format!("{s:#?}"),
            r#"Inlined(
    InlineStr {
        inner: [
            232,
            151,
            143,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        len: 3,
    },
)"#
        );
    }

    #[test]
    fn impl_clone_boxed_long() {
        let s = "this string won't fit in a box".to_owned();
        let s: CowStr = s.into();
        if let CowStr::Boxed(_) = s {
        } else {
            panic!("Expected Boxed case");
        }

        let s2 = s.clone();
        assert_eq!(s.deref(), s2.deref());

        if let CowStr::Boxed(_) = s2 {
        } else {
            panic!("Expected Boxed clone");
        }
    }

    #[test]
    fn impl_clone_borrowed() {
        let s = "this long string is borrowed";
        let s: CowStr = s.into();
        if let CowStr::Borrowed(_) = s {
        } else {
            panic!("Expected Borrowed case");
        }

        let s2 = s.clone();
        assert_eq!(s.deref(), s2.deref());

        if let CowStr::Borrowed(_) = s2 {
        } else {
            panic!("Expected Borrowed clone");
        }
    }

    #[test]
    fn impl_clone_inlined() {
        let s: CowStr = 's'.into();
        if let CowStr::Inlined(_) = s {
        } else {
            panic!("Expected Inlined case");
        }

        let s2 = s.clone();
        assert_eq!(s.deref(), s2.deref());

        if let CowStr::Inlined(_) = s2 {
        } else {
            panic!("Expected Inlined clone");
        }
    }

    #[test]
    fn impl_hash() {
        use std::{
            collections::hash_map::DefaultHasher,
            hash::{Hash, Hasher},
        };

        let mut hasher = DefaultHasher::new();
        "🍔".hash(&mut hasher);
        let expected = hasher.finish();

        let s: CowStr = '🍔'.into();
        if let CowStr::Inlined(_) = s {
        } else {
            panic!("Expected Inlined case");
        }
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        let actual = hasher.finish();
        assert_eq!(expected, actual);

        let s = CowStr::Borrowed("🍔");
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        let actual = hasher.finish();
        assert_eq!(expected, actual);

        let s = "🍔".to_owned();
        let s: CowStr = s.into();
        if let CowStr::Boxed(_) = s {
        } else {
            panic!("Expected Boxed case");
        }
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        assert_eq!(expected, actual);
    }

    #[test]
    fn impl_from_str() {
        let s = "xyz";
        let s: CowStr = s.into();
        assert_eq!(s.deref(), "xyz");

        if let CowStr::Borrowed(_) = s {
        } else {
            panic!("Expected Borrowed case");
        }
    }
}
