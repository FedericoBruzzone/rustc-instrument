# rustc-instrument

A crate to instrument the Rust compiler (`rustc`).

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
cargo run --bin print-hir-ast-driver -- ./test-crate/src/main.rs # the `--` are optional
# cargo run --bin print-hir-ast-driver -- --cfg 'feature="test"' ./test-crate/src/main.rs
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

## General Information

1. This crate is a fork of the [rustc_plugin](https://github.com/cognitive-engineering-lab/rustc_plugin) crate.
2. A common issue when using this crate is the error:
    ```
    error: failed to run `rustc` to learn about target-specific information

    Caused by:
      could not execute process `/home/fcb/dev/rustc-instrument/target/debug/pprint-hir-ast-driver
            /home/fcb/.rustup/toolchains/nightly-2024-01-24-x86_64-unknown-linux-gnu/bin/rustc -
            --crate-name ___
            --print=file-names
            --crate-type bin
            --crate-type rlib
            --crate-type dylib
            --crate-type cdylib
            --crate-type staticlib
            --crate-type proc-macro
            --print=sysroot
            --print=split-debuginfo
            --print=crate-name
            --print=cfg` (never executed)

    Caused by:
      No such file or directory (os error 2)
    ```
    this is due to the fact `pprint-hir-ast-driver` is not a correct name for the executable (it should be `print-hir-ast-driver` in this example).
    Make sure to set in the `driver_name` function of the `RustcPlugin` trait the correct name of the executable.
    ```rust
    fn driver_name(&self) -> Cow<'static, str> {
        "print-hir-ast-driver".into()
    }
    ```

## Contact

If you have any questions, suggestions, or feedback, do not hesitate to [contact me](https://federicobruzzone.github.io/).

