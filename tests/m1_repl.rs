mod common;

use common::{kvs, lines};

fn run(input: &str) -> Vec<String> {
    let out = kvs(&[], input);
    assert!(out.status.success(), "kvs exited with {:?}", out.status);
    lines(&out)
}

#[test]
fn set_then_get() {
    assert_eq!(run("set a 1\nget a\n"), ["OK", "1"]);
}

#[test]
fn get_missing_prints_not_found() {
    assert_eq!(run("get nope\n"), ["NOT_FOUND"]);
}

#[test]
fn overwrite_keeps_latest() {
    assert_eq!(run("set a 1\nset a 2\nget a\n"), ["OK", "OK", "2"]);
}

#[test]
fn rm_existing_then_get() {
    assert_eq!(run("set a 1\nrm a\nget a\n"), ["OK", "OK", "NOT_FOUND"]);
}

#[test]
fn rm_missing_prints_not_found() {
    assert_eq!(run("rm a\n"), ["NOT_FOUND"]);
}

#[test]
fn value_keeps_spaces() {
    assert_eq!(
        run("set greeting hello big world\nget greeting\n"),
        ["OK", "hello big world"]
    );
}

#[test]
fn unicode_keys_and_values() {
    assert_eq!(run("set ☃ ❄️ snow\nget ☃\n"), ["OK", "❄️ snow"]);
}

#[test]
fn unknown_command_is_err() {
    let out = run("fly a\n");
    assert_eq!(out.len(), 1);
    assert!(out[0].starts_with("ERR"), "got {out:?}");
}

#[test]
fn missing_arguments_are_err() {
    let out = run("set a\nget\nrm\nget a b\n");
    assert_eq!(out.len(), 4, "one reply per command, got {out:?}");
    assert!(out.iter().all(|l| l.starts_with("ERR")), "got {out:?}");
}

#[test]
fn blank_lines_produce_no_output() {
    assert_eq!(run("\n\nset a 1\n\nget a\n\n"), ["OK", "1"]);
}

#[test]
fn exits_zero_at_end_of_input() {
    assert!(kvs(&[], "").status.success());
}

#[test]
fn key_limit_counts_bytes_not_characters() {
    // 85 snowmen = 255 bytes (allowed). 86 snowmen = 258 bytes (too long).
    let ok = "☃".repeat(85);
    let too_long = "☃".repeat(86);
    let out = run(&format!("set {ok} v\nset {too_long} v\n"));
    assert_eq!(out[0], "OK");
    assert!(out[1].starts_with("ERR"), "got {out:?}");
}

#[test]
fn value_over_64_kib_is_err() {
    let max = "x".repeat(64 * 1024);
    let over = "x".repeat(64 * 1024 + 1);
    let out = run(&format!("set a {max}\nset b {over}\nget b\n"));
    assert_eq!(out[0], "OK");
    assert!(
        out[1].starts_with("ERR"),
        "got {:?}",
        &out[1][..out[1].len().min(40)]
    );
    assert_eq!(out[2], "NOT_FOUND");
}
