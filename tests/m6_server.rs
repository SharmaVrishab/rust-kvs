//! Milestone 6 — the TCP server (`architecture.md` §5).
//!
//! Given to you like M1–M4. Each test is `#[ignore]`d with the day it
//! belongs to; delete that line when you reach the day. The `Server`
//! helper below is the part that is fiddly to get right (port 0, reading
//! `listening on …`, killing the child on drop) — read it line by line on
//! day 15 before you use it. On day 26 copy it into `tests/m8_async.rs`
//! and pass `&["--runtime", "async"]`.

mod common;

use std::io::{BufRead, BufReader, Write};
use std::net::{Shutdown, TcpStream};
use std::process::{Child, ChildStdout, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use common::TempDir;

/// A running `kvs-server` bound to an OS-chosen port. Killed on drop.
struct Server {
    child: Child,
    addr: String,
    // Kept alive so the server never gets EPIPE if it prints again.
    _stdout: BufReader<ChildStdout>,
}

impl Server {
    /// Start `kvs-server --dir <dir> --addr 127.0.0.1:0 <extra…>` and read
    /// the `listening on <ip:port>` line (§5) to learn the port.
    fn start(dir: &TempDir, extra: &[&str]) -> Self {
        let mut child = Command::new(env!("CARGO_BIN_EXE_kvs-server"))
            .args(["--dir", dir.arg(), "--addr", "127.0.0.1:0"])
            .args(extra)
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("spawn kvs-server");

        let mut stdout = BufReader::new(child.stdout.take().expect("stdout"));
        let mut line = String::new();
        stdout.read_line(&mut line).expect("read first stdout line");
        let addr = line
            .trim()
            .strip_prefix("listening on ")
            .unwrap_or_else(|| {
                panic!("first stdout line must be `listening on <addr>`, got {line:?}")
            })
            .to_owned();

        Server {
            child,
            addr,
            _stdout: stdout,
        }
    }

    fn connect(&self) -> TcpStream {
        let stream = TcpStream::connect(&self.addr).expect("connect to kvs-server");
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .expect("set read timeout");
        stream
    }

    /// Wait up to `within` for the server to exit; return its exit code.
    fn wait_exit(&mut self, within: Duration) -> Option<i32> {
        let deadline = Instant::now() + within;
        while Instant::now() < deadline {
            if let Some(status) = self.child.try_wait().expect("try_wait") {
                return status.code();
            }
            thread::sleep(Duration::from_millis(20));
        }
        None
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

/// Send `input` (one or more lines) and read exactly `replies` reply lines.
fn send(stream: &mut TcpStream, input: &str, replies: usize) -> Vec<String> {
    stream.write_all(input.as_bytes()).expect("write to server");
    stream.flush().expect("flush");
    let mut reader = BufReader::new(stream.try_clone().expect("try_clone"));
    (0..replies)
        .map(|_| {
            let mut line = String::new();
            let n = reader.read_line(&mut line).expect("read reply");
            assert!(n > 0, "server closed the connection early");
            line.trim_end().to_owned()
        })
        .collect()
}

#[test]
#[ignore = "day 15: delete this line to start milestone 6"]
fn set_and_get_over_tcp() {
    let dir = TempDir::new();
    let server = Server::start(&dir, &[]);
    let mut c = server.connect();
    assert_eq!(
        send(&mut c, "set a 1\nget a\nget nope\n", 3),
        ["OK", "1", "NOT_FOUND"]
    );
}

#[test]
#[ignore = "day 16: delete this line when the server is threaded"]
fn two_clients_interleave() {
    let dir = TempDir::new();
    let server = Server::start(&dir, &[]);

    let mut a = server.connect();
    let mut b = server.connect();

    // Both connections are open at once; a single-threaded accept loop
    // would serve `a` to completion before it ever reads from `b`.
    assert_eq!(send(&mut b, "set b 1\n", 1), ["OK"]);
    assert_eq!(send(&mut a, "set a 1\n", 1), ["OK"]);
    assert_eq!(send(&mut b, "get a\n", 1), ["1"]);
    assert_eq!(send(&mut a, "get b\n", 1), ["1"]);
}

#[test]
#[ignore = "day 17: delete this line when `shutdown` exists"]
fn shutdown_exits_zero_with_an_idle_client_connected() {
    let dir = TempDir::new();
    let mut server = Server::start(&dir, &[]);

    let _idle = server.connect(); // never sends anything
    let mut active = server.connect();
    assert_eq!(send(&mut active, "set a 1\nshutdown\n", 2), ["OK", "OK"]);

    assert_eq!(
        server.wait_exit(Duration::from_secs(2)),
        Some(0),
        "server must exit 0 within 2 s even though a client is idle"
    );
}

#[test]
#[ignore = "day 19: delete this line to finish milestone 6"]
fn client_vanishing_mid_command_does_not_crash_the_server() {
    let dir = TempDir::new();
    let server = Server::start(&dir, &[]);

    {
        let mut half = server.connect();
        half.write_all(b"set a ").expect("write half a command");
        half.shutdown(Shutdown::Both).expect("shutdown socket");
    } // dropped: the server sees EOF / reset mid-line

    let mut c = server.connect();
    assert_eq!(send(&mut c, "set b 2\nget b\n", 2), ["OK", "2"]);
}

#[test]
#[ignore = "day 19: delete this line to finish milestone 6"]
fn stats_reports_counts() {
    let dir = TempDir::new();
    let server = Server::start(&dir, &[]);
    let mut c = server.connect();
    let out = send(&mut c, "set a 1\nget a\nget a\nstats\n", 4);
    assert_eq!(&out[..3], ["OK", "1", "1"]);
    assert_eq!(
        out[3], "keys=1 reads=2 writes=1 compactions=0",
        "got {:?}",
        out[3]
    );
}

#[test]
#[ignore = "day 19: delete this line to finish milestone 6"]
fn line_over_the_limit_is_err_and_closes_the_connection() {
    let dir = TempDir::new();
    let server = Server::start(&dir, &[]);
    let mut c = server.connect();

    // §2: longer than 65,797 bytes → `ERR line too long`, then close.
    let huge = format!("set k {}\n", "x".repeat(70_000));
    let out = send(&mut c, &huge, 1);
    assert!(
        out[0].starts_with("ERR"),
        "got {:?}",
        &out[0][..out[0].len().min(40)]
    );

    let mut reader = BufReader::new(c.try_clone().unwrap());
    let mut line = String::new();
    let n = reader.read_line(&mut line).unwrap_or(0);
    assert_eq!(
        n, 0,
        "server must close the connection after an over-long line"
    );
}

#[test]
#[ignore = "day 19: delete this line to finish milestone 6"]
fn hundred_clients_hundred_ops_each() {
    const CLIENTS: usize = 100;
    const OPS: usize = 100;

    let dir = TempDir::new();
    let server = Server::start(&dir, &[]);

    let handles: Vec<_> = (0..CLIENTS)
        .map(|c| {
            let mut stream = server.connect();
            thread::spawn(move || {
                for i in 0..OPS {
                    let key = format!("c{c}k{i}");
                    let val = format!("v{c}_{i}");
                    let out = send(&mut stream, &format!("set {key} {val}\nget {key}\n"), 2);
                    assert_eq!(out, ["OK".to_owned(), val], "client {c} op {i}");
                }
            })
        })
        .collect();

    for h in handles {
        h.join().expect("client thread panicked");
    }

    let mut c = server.connect();
    assert_eq!(
        send(&mut c, "get c0k0\nget c99k99\n", 2),
        ["v0_0", "v99_99"]
    );
}
