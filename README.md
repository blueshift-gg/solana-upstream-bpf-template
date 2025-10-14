# Solana Upstream BPF Template

A template for creating Solana BPF programs using the upstream LLVM toolchain.

## Prerequisites

Install the required tools:

```bash
# Install sbpf-linker
cargo install sbpf-linker

# Install cargo-generate
cargo install cargo-generate
```

## Usage

Create a new project from this template:

```bash
cargo generate --git https://github.com/blueshift-gg/solana-upstream-bpf-template.git
```

## Building

Build your BPF program:

```bash
cargo +nightly build-bpf
```

The compiled program will be at:
```
target/bpfel-unknown-none/release/lib{{crate_name}}.so
```

## Testing

Run tests:

```bash
cargo test
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
