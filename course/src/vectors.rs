pub fn run() {
    let mut vec: Vec<i32> = vec![1, 2, 3];
    println!("{:?}", vec);

    vec[0] = 3;
    println!("{:?}", vec);
    vec.push(10);
    vec.push(11);
    vec.push(12);

    println!("{:?}", vec);

    vec.pop();
    println!("{:?}", vec);

    for val in vec.iter() {
        println!("{}", val);
    }

    for x in vec.iter_mut() {
        // derefrence??
        *x = *x * *x;
    }

    println!("{:?}", vec);
}
