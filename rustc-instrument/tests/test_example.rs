use std::{env, fs, path::Path, process::Command, sync::Once};
use anyhow::{ensure, Context, Result};

static SETUP: Once = Once::new();

fn run(dir: &str, f: impl FnOnce(&mut Command)) -> Result<String> {
    let root = env::temp_dir().join("rustc_plugin");

    let heredir = Path::new(".").canonicalize()?;

    SETUP.call_once(|| {
        let mut cmd = Command::new("cargo");
        cmd.args([
            "install",
            "--path",
            "../examples/print-hir-ast",
            "--debug",
            "--locked",
            "--root",
        ]);
        cmd.arg(&root);
        cmd.current_dir(&heredir);
        let status = cmd.status().unwrap();
        if !status.success() {
            panic!("installing example failed")
        }
    });

    let mut cmd = Command::new("cargo");
    cmd.arg("print-hir-ast");

    let path = format!(
        "{}:{}",
        root.join("bin").display(),
        env::var("PATH").unwrap_or_else(|_| "".into())
    );
    cmd.env("PATH", path);

    let ws = heredir.join("tests").join(dir);
    cmd.current_dir(&ws);

    f(&mut cmd);

    let _ = fs::remove_dir_all(ws.join("target"));

    let output = cmd.output().context("Process failed")?;
    ensure!(
        output.status.success(),
        "Process exited with non-zero exit code. Stderr:\n{}",
        String::from_utf8(output.stderr)?
    );

    Ok(String::from_utf8(output.stdout)?)
}

// TODO: why do these tests need to be run sequentially?
// cargo test -- --test-threads=1

#[test]
fn basic() -> Result<()> {
    let output = run("workspaces/basic", |_cmd| {})?;
    // println!("{}", output); // cargo test -- --nocapture
    assert!(output.contains(r#"ident: add#0"#));
    Ok(())
}

#[test]
fn arg() -> Result<()> {
    let output = run("workspaces/basic", |cmd| {
        cmd.arg("--allcaps");
    })?;
    // println!("{}", output); // cargo test -- --nocapture
    assert!(output.contains(r#"IDENT: ADD#0"#));
    Ok(())
}

#[test]
fn feature() -> Result<()> {
    let output = run("workspaces/basic", |cmd| {
        cmd.args(["--", "--features", "sub"]);
    })?;
    assert!(
        output.contains(r#"symbol: "sub""#),
        "output:\n{output}"
    );
    Ok(())
}

#[test]
fn multi() -> Result<()> {
    run("workspaces/multi", |_cmd| {})?;
    Ok(())
}
