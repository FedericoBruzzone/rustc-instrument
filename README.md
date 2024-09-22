# rustc-instrument

A wrapper on `rustc` to manipulate the raw Rust AST before expansion at compile time.

## Examples

### Using `cli` of `rustc-instrument`

It will print the AST of all the the crates in the current workspace.

Note that the `cli` will use internally the `driver`.

```bash
cargo clean
cargo build
RUST_LOG=debug cargo run --bin cargo-print-hir-ast
```

### Using `driver` of `rustc-instrument`

It will print the AST of the crate at the given path, and generate the executable.

Note that `print-hir-ast-driver` is a wrapper on top of `rustc`. If `CARGO_PRIMARY_PACKAGE` is not set then `rustc-plug-ast-driver` is equivalent to `rustc`.


**Simple Way**

```bash
export CARGO_PRIMARY_PACKAGE=1
cargo clean
cargo run --bin print-hir-ast-driver ./test-crate/src/main.rs
```

To specify the path of the crate, use the following command:
> In `--extern env_logger=./target/debug/deps/libenv_logger-<HASH>.rlib` replace `<HASH>` with the actual hash of the `env_logger` crate.
```bash
# -L dependency=./target/debug/deps --extern env_logger=./target/debug/deps/libenv_logger-<HASH>.rlib
cargo run --bin print-hir-ast-driver ./test-crate/src/main.rs -L dependency=./target/debug/deps --extern env_logger=./target/debug/deps/libenv_logger-<HASH>.rlib
```

**Advanced Way**

```bash
export CARGO_PRIMARY_PACKAGE=1
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:<`rustc --print sysroot`>/lib # Something like ~/.rustup/toolchains/nightly-2024-01-24-x86_64-unknown-linux-gnu/lib
cargo clean
cargo build # build the compiler
RUST_LOG=debug ./target/debug/print-hir-ast-driver ./test-crate/src/main.rs
```

To specify the path of the crate, use the following command:
> In `--extern env_logger=./target/debug/deps/libenv_logger-<HASH>.rlib` replace `<HASH>` with the actual hash of the `env_logger` crate.
```bash
# -L dependency=./target/debug/deps --extern env_logger=./target/debug/deps/libenv_logger-<HASH>.rlib
RUST_LOG=debug ./target/debug/print-hir-ast-driver ./test-crate/src/main.rs -L dependency=./target/debug/deps --extern env_logger=./target/debug/deps/libenv_logger-<HASH>.rlib
```

## Contact

If you have any questions, suggestions, or feedback, do not hesitate to [contact me](https://federicobruzzone.github.io/).

