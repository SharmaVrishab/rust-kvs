fn main() {
    println!("{}", is_even(20))
}

fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    return false;
}

fn fib(num: i32) -> i32 {
    let mut first = 0;
    let mut second = 1;
    if num == 0 {
        return first;
    }
    if num == 1 {
        return 1;
    }

    for i in 1..num - 2 {
        let temp = second;
        second = second + first;
        first = temp;
    }
    return second;
}
