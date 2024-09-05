# rustc-plug-ast

A wrapper on `rustc` to manipulate the raw Rust AST before expansion at compile time.

## Examples

### Using `driver` of `rustc_plugin`

It will print the AST of the crate at the given path, and generate the executable.

Note that `rustc-plug-ast-driver` is a wrapper on top of `rustc`. If `CARGO_PRIMARY_PACKAGE` is not set then `rustc-plug-ast-driver` is equivalent to `rustc`.

> In `--extern env_logger=./target/debug/deps/libenv_logger-<HASH>.rlib` replace `<HASH>` with the actual hash of the `env_logger` crate.

**Simple Way**

```bash
export CARGO_PRIMARY_PACKAGE=1
cargo clean
cargo run --bin rustc-plug-ast-driver ./test-crate/src/main.rs -L dependency=./target/debug/deps --extern env_logger=./target/debug/deps/libenv_logger-<HASH>.rlib
```

**Advanced Way**

```bash
export CARGO_PRIMARY_PACKAGE=1
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:<`rustc --print sysroot`>/lib # Something like ~/.rustup/toolchains/nightly-2024-01-24-x86_64-unknown-linux-gnu/lib
cargo clean
./target/debug/rustc-plug-ast-driver ./test-crate/src/main.rs -L dependency=./target/debug/deps --extern env_logger=./target/debug/deps/libenv_logger-<HASH>.rlib
```

### Using `cli` of `rustc_plugin`

It will print the AST of all the the crates in the current workspace.

```bash
cargo clean
cargo run --bin rustc-plug-ast-cli
```

## Contact

If you have any questions, suggestions, or feedback, do not hesitate to [contact me](https://federicobruzzone.github.io/).

