use std::time::Instant;

fn main() {
    let now = Instant::now();

    let mut x: i64 = 0;

    for i in 1..100_000_00 {
        x += i;
    }

    println!("TIME: {}", now.elapsed().as_secs());
    println!("NUMBER: {}", x);
}
