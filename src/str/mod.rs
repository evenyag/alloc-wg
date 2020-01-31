//! Unicode string slices.
//!
//! *[See also the `str` primitive type][str].*
//!
//! The `&str` type is one of the two main string types, the other being `String`.
//! Unlike its `String` counterpart, its contents are borrowed.
//!
//! # Basic Usage
//!
//! A basic string declaration of `&str` type:
//!
//! ```
//! # #![allow(unused_variables)]
//! let hello_world = "Hello, World!";
//! ```
//!
//! Here we have declared a string literal, also known as a string slice.
//! String literals have a static lifetime, which means the string `hello_world`
//! is guaranteed to be valid for the duration of the entire program.
//! We can explicitly specify `hello_world`'s lifetime as well:
//!
//! ```
//! # #![allow(unused_variables)]
//! let hello_world: &'static str = "Hello, world!";
//! ```
//!
//! [str]: https://doc.rust-lang.org/nightly/std/primitive.str.html

use crate::{
    alloc::DeallocRef,
    borrow::{Borrow, BorrowMut},
    boxed::Box,
    string::String,
};
pub use liballoc::str::{
    from_utf8,
    from_utf8_mut,
    from_utf8_unchecked,
    from_utf8_unchecked_mut,
    Chars,
    FromStr,
    Utf8Error,
};

pub mod lossy;

#[rustfmt::skip]
// https://tools.ietf.org/html/rfc3629
static UTF8_CHAR_WIDTH: [u8; 256] = [
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x1F
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x3F
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x5F
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x7F
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, // 0x9F
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, // 0xBF
0,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,
2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2, // 0xDF
3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3, // 0xEF
4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0, // 0xFF
];

/// Given a first byte, determines how many bytes are in this UTF-8 character.
#[inline]
pub fn utf8_char_width(b: u8) -> usize {
    UTF8_CHAR_WIDTH[b as usize] as usize
}

impl<D: DeallocRef> Borrow<str> for String<D> {
    #[inline]
    fn borrow(&self) -> &str {
        &self[..]
    }
}

impl<D: DeallocRef> BorrowMut<str> for String<D> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut str {
        &mut self[..]
    }
}

/// Converts a boxed slice of bytes to a boxed string slice without checking
/// that the string contains valid UTF-8.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn from_boxed_utf8_unchecked<A: DeallocRef>(v: Box<[u8], A>) -> Box<str, A> {
    let a = core::ptr::read(v.build_alloc());
    Box::from_raw_in(Box::into_raw(v) as *mut str, a)
}
