# Spyro: Reignited Auto-Splitter

A cross-platform auto-splitter for Spyro: Reignited Trilogy.

> [!warning]
> 
> This branch is an in-development version of the auto-splitter, re-written from the ground up to make the code more readable and maintainable. A working version of the auto-splitter can be found on the [`legacy`](https://github.com/KatieZeldaKat/spyro-rt-autosplitter/tree/legacy) branch.

## Compilation

This auto splitter is written in Rust. In order to compile it, you need to
install the Rust compiler: [Install Rust](https://www.rust-lang.org/tools/install).

Afterwards install the WebAssembly target:

```bash
rustup target add wasm32-unknown-unknown
```

The auto splitter can now be compiled:

```bash
cargo b --release --target wasm32-unknown-unknown
```

The auto splitter is then available at:

```
target/wasm32-unknown-unknown/release/spyro_rt_autosplitter.wasm
```

## Development

Make sure to look into the [API documentation](https://livesplit.org/asr/asr/) for the `asr` crate.

You can use the [debugger](https://github.com/LiveSplit/asr-debugger) while
developing the auto splitter to more easily see the log messages, statistics,
dump memory, step through the code and more.
