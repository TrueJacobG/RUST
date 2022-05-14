pub fn run() {
    let age = 20;

    if age > 18 {
        println!("yes");
    } else {
        println!("no");
    }

    let is_full_age = if age >= 18 { true } else { false };

    println!("Is fullage: {}", is_full_age);
}
