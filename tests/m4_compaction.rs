mod common;

use common::{run_dir, TempDir};

const WRITES: usize = 3_000;
const VALUE_LEN: usize = 1_000;

fn value(i: usize) -> String {
    format!("{i:0>VALUE_LEN$}")
}

fn overwrite_same_key(dir: &TempDir) {
    let mut input = String::new();
    for i in 0..WRITES {
        input.push_str(&format!("set k {}\n", value(i)));
    }
    let out = run_dir(dir, &input);
    assert_eq!(out.len(), WRITES);
}

#[test]
#[ignore = "day 12: delete this line to start milestone 4"]
fn log_compacts_after_one_mib_of_stale_data() {
    let dir = TempDir::new();
    overwrite_same_key(&dir);
    let size = std::fs::metadata(dir.path().join("kvs.log")).unwrap().len();
    // ~3 MB was written. After compaction at most ~1 MiB of stale data remains.
    assert!(
        size < 1_100_000,
        "kvs.log is {size} bytes, expected compaction"
    );
}

#[test]
#[ignore = "day 12: delete this line to start milestone 4"]
fn data_is_correct_after_compaction_and_restart() {
    let dir = TempDir::new();
    run_dir(&dir, "set other keep-me\n");
    overwrite_same_key(&dir);
    assert_eq!(
        run_dir(&dir, "get k\nget other\n"),
        [value(WRITES - 1), "keep-me".to_string()]
    );
}

#[test]
#[ignore = "day 12: delete this line to start milestone 4"]
fn no_compaction_leftovers() {
    let dir = TempDir::new();
    overwrite_same_key(&dir);
    let names: Vec<String> = std::fs::read_dir(dir.path())
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    assert_eq!(names, ["kvs.log"], "only the log should remain");
}
