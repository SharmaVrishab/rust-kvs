#![allow(dead_code)]

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

static NEXT: AtomicUsize = AtomicUsize::new(0);

/// A directory that deletes itself when it goes out of scope.
pub struct TempDir(PathBuf);

impl TempDir {
    pub fn new() -> Self {
        let n = NEXT.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!("kvs-test-{}-{n}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("create temp dir");
        TempDir(path)
    }

    pub fn path(&self) -> &Path {
        &self.0
    }

    pub fn arg(&self) -> &str {
        self.0.to_str().expect("temp path is UTF-8")
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Run the `kvs` binary with `args`, feed `input` on stdin, wait for it to exit.
pub fn kvs(args: &[&str], input: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_kvs"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn kvs");

    // Writing stdin on another thread: if we wrote it all first, a child that
    // fills its stdout pipe would block, and so would we.
    let mut stdin = child.stdin.take().expect("stdin");
    let input = input.to_owned();
    let writer = thread::spawn(move || {
        let _ = stdin.write_all(input.as_bytes());
    });

    let output = child.wait_with_output().expect("wait for kvs");
    writer.join().expect("stdin writer");
    output
}

pub fn lines(output: &Output) -> Vec<String> {
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::to_owned)
        .collect()
}

pub fn run_dir(dir: &TempDir, input: &str) -> Vec<String> {
    let out = kvs(&["--dir", dir.arg()], input);
    assert!(
        out.status.success(),
        "kvs exited with {:?}, stderr: {}",
        out.status,
        String::from_utf8_lossy(&out.stderr)
    );
    lines(&out)
}
