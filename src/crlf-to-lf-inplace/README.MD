# crlf-to-lf-inplace

[![Crates.io](https://img.shields.io/crates/v/crlf-to-lf-inplace.svg)](https://crates.io/crates/crlf-to-lf-inplace)
[![Docs.rs](https://docs.rs/crlf-to-lf-inplace/badge.svg)](https://docs.rs/crlf-to-lf-inplace)
[![CI](https://github.com/Sewer56/crlf-to-lf-inplace/actions/workflows/rust.yml/badge.svg)](https://github.com/Sewer56/crlf-to-lf-inplace/actions)

Fast in-place CRLF to LF line ending conversion for Rust strings. Uses memchr for good performance without custom SIMD.

## Features

- No-std compatible with optional std support
- Zero-allocation in-place conversion
- Fast CRLF detection using memchr
- UTF-8 safe conversion

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
crlf-to-lf-inplace = "0.2.0"
```

No-std builds can disable the default `std` feature:

```toml
[dependencies]
crlf-to-lf-inplace = { version = "0.2.0", default-features = false }
```

### Feature Flags

| Feature | Description                                                                      |
| ------- | -------------------------------------------------------------------------------- |
| `std`   | Enable standard library support (disabled by default for `no_std` compatibility) |

## Usage

Use [`crlf_to_lf_inplace`] for `String` values, or [`crlf_to_lf_inplace_utf8_bytes`] for raw UTF-8 byte buffers. The byte-slice API returns the new logical length, so truncate or slice the buffer using the returned length.

### Basic Example

```rust
use crlf_to_lf_inplace::crlf_to_lf_inplace;

let mut text = String::from("line1\r\nline2\r\n");
crlf_to_lf_inplace(&mut text);
assert_eq!(text, "line1\nline2\n");
```

### Advanced Example

```rust
use crlf_to_lf_inplace::crlf_to_lf_inplace_utf8_bytes;

let mut buffer = b"foo\r\nbar\r\nbaz\r\n".to_vec();
let new_len = crlf_to_lf_inplace_utf8_bytes(&mut buffer);
buffer.truncate(new_len);
assert_eq!(buffer.as_slice(), b"foo\nbar\nbaz\n");
```

## Performance

Benchmarked on AMD Ryzen 9 9950X3D (single-threaded) with ~71 KiB of concatenated Markdown
files (29 files from real opencode configuration):

| Input Type               | Time    | Throughput |
| ------------------------ | ------- | ---------- |
| CRLF (conversion needed) | 9.72 µs | 7.2 GiB/s  |
| LF only (no conversion)  | 378 ns  | 179 GiB/s  |

The implementation uses `memchr` for fast `\r` scanning with a two-phase approach:
first locating the initial CRLF, then processing remaining matches. Files already
using LF line endings benefit from an early-exit path.

Run benchmarks with:

```bash
cargo bench
```

## UTF-8 Safety

This crate only handles Rust's standard UTF-8 encoding.

The 'trick' is below:

| Code point range   | Byte 1     | Byte 2     | Byte 3     | Byte 4     | Code points |
| ------------------ | ---------- | ---------- | ---------- | ---------- | ----------- |
| U+0000 - U+007F    | `0xxxxxxx` |            |            |            | 128         |
| U+0080 - U+07FF    | `110xxxxx` | `10xxxxxx` |            |            | 1920        |
| U+0800 - U+FFFF    | `1110xxxx` | `10xxxxxx` | `10xxxxxx` |            | 61440       |
| U+10000 - U+10FFFF | `11110xxx` | `10xxxxxx` | `10xxxxxx` | `10xxxxxx` | 1048576     |

Bytes `0x00-0x7F` (ASCII) are always single-byte UTF8.  
Bytes `0x00-0x7F` cannot be follow-up bytes.

That means our lovely friends `\r` (0x0D) and `\n` (0x0A) can be easily be found
by searching their single byte directly. 😉

## License

Licensed under MIT
