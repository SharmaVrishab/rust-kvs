use std::env;

fn main() {
    // {
    //     let n: u8 = 255;
    //     let m = n + 1;
    //     let _ = m;
    // }

    let args: Vec<String> = env::args().collect();

    let input = &args[1];

    // let n = input.parse::<u8>().unwrap();
    let n = match input.parse::<u8>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Usage: {} <u8>", args[0]);
            std::process::exit(1);
        }
    };

    println!("{}", n + 1);
    println!("{:?}", n.checked_add(1));
    println!("{}", n.wrapping_add(1));
    println!("{}", n.saturating_add(1));
}
