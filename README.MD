# crlf-to-lf-inplace

[![Crates.io](https://img.shields.io/crates/v/crlf-to-lf-inplace.svg)](https://crates.io/crates/crlf-to-lf-inplace)
[![Docs.rs](https://docs.rs/crlf-to-lf-inplace/badge.svg)](https://docs.rs/crlf-to-lf-inplace)
[![CI](https://github.com/Sewer56/crlf-to-lf-inplace/actions/workflows/rust.yml/badge.svg)](https://github.com/Sewer56/crlf-to-lf-inplace/actions)

Fast in-place CRLF to LF line ending conversion for Rust strings. Uses memchr for good performance without custom SIMD.

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
crlf-to-lf-inplace = "0.1.0"
```

```rust
use crlf_to_lf_inplace::crlf_to_lf_inplace;

let mut text = String::from("line1\r\nline2\r\n");
crlf_to_lf_inplace(&mut text);
assert_eq!(text, "line1\nline2\n");
```
See the [crlf-to-lf-inplace crate documentation](./src/crlf-to-lf-inplace/README.MD) for detailed usage.

## Performance

Benchmarked on AMD Ryzen 9 9950X3D (single-threaded) with ~71 KiB of concatenated Markdown
files (29 files from real documentation):

| Input Type               | Time    | Throughput |
| ------------------------ | ------- | ---------- |
| CRLF (conversion needed) | 9.72 µs | 7.2 GiB/s  |
| LF only (no conversion)  | 378 ns  | 179 GiB/s  |

The benchmark data consists of real-world Markdown documentation files, providing a
realistic mix of text content and line endings.

## Crates

- [crlf-to-lf-inplace](./src/crlf-to-lf-inplace/README.MD): In-place CRLF → LF conversion for strings/byte slices with a no-std friendly core.

## Developer Manual

For step-by-step development guidance, see the [Developer Manual](https://reloaded-project.github.io/reloaded-templates-rust/manual/).

## Contributing

We welcome contributions! See the [Contributing Guide](https://reloaded-project.github.io/reloaded-templates-rust/manual/#contributing) for details.

## License

Licensed under [MIT](./LICENSE).

[codecov]: https://app.codecov.io
[gh-actions-secrets]: https://github.com/Sewer56/crlf-to-lf-inplace/settings/secrets/actions
[gh-actions-dependabot]: https://github.com/Sewer56/crlf-to-lf-inplace/settings/secrets/dependabot
[gh-pages]: https://github.com/Sewer56/crlf-to-lf-inplace/settings/pages
[crates-io-key]: https://crates.io/settings/tokens
[nuget-key]: https://www.nuget.org/account/apikeys
