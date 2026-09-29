mod common;

use common::{kvs, run_dir, TempDir};

#[test]
#[ignore = "day 6: delete this line to start milestone 2"]
fn data_survives_restart() {
    let dir = TempDir::new();
    assert_eq!(run_dir(&dir, "set a 1\nset b two words\n"), ["OK", "OK"]);
    assert_eq!(run_dir(&dir, "get a\nget b\n"), ["1", "two words"]);
}

#[test]
#[ignore = "day 6: delete this line to start milestone 2"]
fn rm_survives_restart() {
    let dir = TempDir::new();
    run_dir(&dir, "set a 1\n");
    assert_eq!(run_dir(&dir, "rm a\n"), ["OK"]);
    assert_eq!(run_dir(&dir, "get a\n"), ["NOT_FOUND"]);
}

#[test]
#[ignore = "day 6: delete this line to start milestone 2"]
fn dir_is_created_if_missing() {
    let root = TempDir::new();
    let nested = root.path().join("does/not/exist/yet");
    let nested = nested.to_str().unwrap();
    let out = kvs(&["--dir", nested], "set a 1\n");
    assert!(out.status.success());
    let out = kvs(&["--dir", nested], "get a\n");
    assert_eq!(String::from_utf8_lossy(&out.stdout).trim(), "1");
}

#[test]
#[ignore = "day 6: delete this line to start milestone 2"]
fn two_dirs_are_independent() {
    let one = TempDir::new();
    let two = TempDir::new();
    run_dir(&one, "set a 1\n");
    assert_eq!(run_dir(&two, "get a\n"), ["NOT_FOUND"]);
}

#[test]
#[ignore = "day 6: delete this line to start milestone 2"]
fn bad_flags_exit_with_code_2() {
    assert_eq!(kvs(&["--nope"], "").status.code(), Some(2));
    assert_eq!(kvs(&["--dir"], "").status.code(), Some(2));
}
