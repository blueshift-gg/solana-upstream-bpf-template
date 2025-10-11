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
target/bpfel-unknown-none/release/libyour_program_name.so
```

## Testing

Run tests:

```bash
cargo test
```

## License

MIT
