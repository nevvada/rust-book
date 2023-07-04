fn fib(num: i16) -> i16 {
    if num == 0 {
        return 1;
    }

    if num == 1 {
        return 1;
    }

    return fib(num - 1) + fib(num - 2);
}

fn main() {
    let fib_number_at_index: i16 = fib(4);

    println!("fib number at index: {}", fib_number_at_index);
}
