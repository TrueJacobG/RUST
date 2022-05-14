pub fn run() {
    // type; length
    let numbers: [i32; 3] = [1, 2, 3];
    println!("{:?}", numbers);

    let mut num2: [i16; 3] = [3, 2, 1];
    num2[0] = 1;
    println!("{:?}", num2);
    println!("Len: {}", num2.len());

    // slice
    let slice: &[i32] = &numbers[0..2];
    println!("{:?}", slice);
}
