use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    // let n = args[1].parse::<u64>().unwrap();
    let input = &args[1];
    let n = match input.parse::<u64>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Usage: {} <u64>", args[0]);
            std::process::exit(1);
        }
    };
    let mut total = 0u64;
    for i in 1..=n {
        total += i;
    }
    println!("{total}");
    let mut total = 0u64;
    let mut i = 1u64;

    while i <= n {
        total += i;
        i += 1;
    }
    println!("{total}");

    let mut total = 0u64;
    let mut i = 1u64;

    let result = loop {
        if i > n {
            break total;
        }

        total += i;
        i += 1;
    };
    println!("{result}");

    let total = (1..=n).sum::<u64>();
    println!("{total}")
}
