mod fibonacci;

use fibonacci::Fibonacci;

use std::time::Instant;

fn main() {
    const N: u64 = 40;
    let mut fib = Fibonacci::new();

    println!("Calculating fib({}) with cache: ", N);
    let start = Instant::now();
    println!("{}", fib.calc(N));
    let end = Instant::now();
    println!("Took {:?}", end.duration_since(start));

    println!("Calculating fib({}) without cache: ", N);
    let start = Instant::now();
    println!("{}", fib.calc_no_cache(N));
    let end = Instant::now();
    println!("Took {:?}", end.duration_since(start));
}
