enum Dirs {
    Up,
    Down,
    Left,
    Right,
}

fn move_character(d: Dirs) {
    match d {
        Dirs::Up => println!("Move up!"),
        Dirs::Down => println!("Move down"),
        Dirs::Left => println!("Move left!"),
        Dirs::Right => println!("Move right!"),
    }
}

pub fn run() {
    let p = Dirs::Right;

    move_character(p);
}
