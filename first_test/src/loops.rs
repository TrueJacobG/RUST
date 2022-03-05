pub fn run() {
    let mut i = 0;

    loop {
        println!("i: {}", i);
        i += 1;

        if i == 10 {
            break;
        }
    }

    //FizzBuzz();

    for x in 0..10 {
        println!("x: {}", x - 10);
    }
}

fn FizzBuzz() {
    let mut i = 0;
    while (i <= 100) {
        i += 1;

        if (i % 15 == 0) {
            println!("FizzBuzz");
            continue;
        }

        if (i % 5 == 0) {
            println!("Buzz");
            continue;
        }

        if (i % 3 == 0) {
            println!("Fizz");
            continue;
        }

        println!("{}", i);
    }
}
