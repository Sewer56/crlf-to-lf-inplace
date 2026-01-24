# crlf-to-lf-inplace

Fast in-place CRLF to LF line ending conversion for Rust strings. Uses memchr for decent performance without custom SIMD.

# Project Structure

- `crlf-to-lf-inplace/` - Main library crate
  - `src/` - Library source code
  - `benches/` - Benchmarks

# Code Guidelines

- Optimize for performance; use zero-cost abstractions, avoid allocations.
- Keep modules under 500 lines (excluding tests); split if larger.
- Place `use` inside functions only for `#[cfg]` conditional compilation.
- Prefer `core` over `std` where possible (`core::mem` over `std::mem`).

# Documentation Standards

- Document public items with `///`
- Add examples in docs where helpful
- Use `//!` for module-level docs
- Focus comments on "why" not "what"
- Use [`TypeName`] rustdoc links, not backticks.

# Post-Change Verification

Always run `.cargo/verify.sh` (or `.cargo/verify.ps1` on Windows) after changing code.
