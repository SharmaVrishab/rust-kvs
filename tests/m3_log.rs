mod common;

use std::fs::OpenOptions;
use std::io::Write;

use common::{run_dir, TempDir};

#[test]
#[ignore = "day 10: delete this line to start milestone 3"]
fn writes_go_to_kvs_log() {
    let dir = TempDir::new();
    run_dir(&dir, "set a 1\n");
    let log = dir.path().join("kvs.log");
    assert!(log.exists(), "expected {}", log.display());
    assert!(std::fs::metadata(log).unwrap().len() > 0);
}

#[test]
#[ignore = "day 10: delete this line to start milestone 3"]
fn latest_value_wins_across_restarts() {
    let dir = TempDir::new();
    run_dir(&dir, "set a 1\nset a 2\n");
    run_dir(&dir, "set a 3\n");
    assert_eq!(run_dir(&dir, "get a\n"), ["3"]);
}

#[test]
#[ignore = "day 10: delete this line to start milestone 3"]
fn rm_is_replayed_on_startup() {
    let dir = TempDir::new();
    run_dir(&dir, "set a 1\nset b 2\nrm a\n");
    assert_eq!(run_dir(&dir, "get a\nget b\n"), ["NOT_FOUND", "2"]);
}

#[test]
#[ignore = "day 10: delete this line to start milestone 3"]
fn torn_tail_is_cut_off_and_log_keeps_working() {
    let dir = TempDir::new();
    run_dir(&dir, "set a 1\nset b 2\n");

    // Simulate a crash halfway through writing a record.
    let mut log = OpenOptions::new()
        .append(true)
        .open(dir.path().join("kvs.log"))
        .unwrap();
    log.write_all(&[0xde, 0xad, 0xbe, 0xef, 0x01, 0x02])
        .unwrap();
    drop(log);

    assert_eq!(run_dir(&dir, "get a\nget b\nset c 3\n"), ["1", "2", "OK"]);
    // If the garbage was not cut off, `c` sits after it and is unreadable now.
    assert_eq!(run_dir(&dir, "get c\n"), ["3"]);
}
