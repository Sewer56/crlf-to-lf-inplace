#![doc = include_str!(concat!("../", env!("CARGO_PKG_README")))]
#![no_std]

extern crate alloc;

use alloc::string::String;
use memchr::memchr_iter;

/// Convert CRLF sequences to LF in-place within a byte slice.
///
/// Returns the new length after in-place compaction.
#[inline]
pub fn crlf_to_lf_inplace_bytes(buf: &mut [u8]) -> usize {
    let len = buf.len();
    if len < 2 {
        return len;
    }

    let ptr = buf.as_mut_ptr();
    // memchr_iter only keeps raw pointers and advances its start forward; it does
    // not cache or reuse any data between next() calls. We only write to bytes
    // strictly before that moving start, so it never reads data we've already
    // compacted. Just in case to prove, tests below cover \r\n chains.
    let mut iter = memchr_iter(b'\r', buf);

    // Find first CRLF
    let first_crlf = loop {
        let Some(cr_pos) = iter.next() else {
            return len;
        };
        // Safety: cr_pos < len (from memchr), cr_pos + 1 checked before access
        if cr_pos + 1 < len && unsafe { *ptr.add(cr_pos + 1) } == b'\n' {
            break cr_pos;
        }
    };

    // Set up for processing
    let mut write = first_crlf;
    // Safety: write < len
    unsafe { *ptr.add(write) = b'\n' };
    write += 1;
    let mut read = first_crlf + 2;

    // Process remaining CRLFs
    for cr_pos in iter {
        // Check if followed by \n
        // Safety: cr_pos < len (from memchr), cr_pos + 1 checked before access
        if cr_pos + 1 < len && unsafe { *ptr.add(cr_pos + 1) } == b'\n' {
            // Copy segment before this CRLF
            let segment_len = cr_pos - read;
            // Safety: overlapping copy is safe - we always write to earlier positions
            unsafe { core::ptr::copy(ptr.add(read), ptr.add(write), segment_len) };
            write += segment_len;
            // Safety: write < len
            unsafe { *ptr.add(write) = b'\n' };
            write += 1;
            read = cr_pos + 2;
        }
    }

    // Copy remaining bytes
    let remaining_len = len - read;
    unsafe { core::ptr::copy(ptr.add(read), ptr.add(write), remaining_len) };
    write + remaining_len
}

/// Convert CRLF sequences to LF in-place within a [`String`].
#[inline]
pub fn crlf_to_lf_inplace(s: &mut String) {
    // Safety: `crlf_to_lf_inplace_bytes` only removes `\r` bytes that are
    // immediately followed by `\n`, so UTF-8 validity is preserved.
    let bytes = unsafe { s.as_mut_vec() };
    let new_len = crlf_to_lf_inplace_bytes(bytes);
    s.truncate(new_len);
}

#[cfg(test)]
mod tests {
    use super::{crlf_to_lf_inplace, crlf_to_lf_inplace_bytes};
    use alloc::string::ToString;

    fn assert_bytes(input: &[u8], expected: &[u8]) {
        let mut buffer = input.to_vec();
        let new_len = crlf_to_lf_inplace_bytes(&mut buffer);
        assert_eq!(new_len, expected.len());
        assert_eq!(&buffer[..new_len], expected);
    }

    fn assert_string(input: &str, expected: &str) {
        let mut value = input.to_string();
        crlf_to_lf_inplace(&mut value);
        assert_eq!(value, expected);
    }

    #[test]
    fn empty_input() {
        assert_bytes(b"", b"");
        assert_string("", "");
    }

    #[test]
    fn no_crlf() {
        assert_bytes(b"line1\nline2\n", b"line1\nline2\n");
        assert_string("line1\nline2\n", "line1\nline2\n");
    }

    #[test]
    fn mixed_crlf_lf() {
        assert_bytes(b"line1\r\nline2\nline3\r\n", b"line1\nline2\nline3\n");
        assert_string("line1\r\nline2\nline3\r\n", "line1\nline2\nline3\n");
    }

    #[test]
    fn cr_not_followed_by_lf() {
        assert_bytes(b"a\rb\r\nc\r", b"a\rb\nc\r");
        assert_string("a\rb\r\nc\r", "a\rb\nc\r");
    }

    #[test]
    fn all_crlf() {
        assert_bytes(b"\r\n\r\n", b"\n\n");
        assert_string("\r\n\r\n", "\n\n");
    }

    #[test]
    fn consecutive_cr_before_lf() {
        // \r\r\n should become \r\n (first \r is lone, second is part of CRLF)
        assert_bytes(b"\r\r\n", b"\r\n");
        assert_string("\r\r\n", "\r\n");
    }

    #[test]
    fn consecutive_crlf_chains() {
        assert_bytes(b"a\r\n\r\n\r\nb", b"a\n\n\nb");
        assert_string("a\r\n\r\n\r\nb", "a\n\n\nb");
    }
}
