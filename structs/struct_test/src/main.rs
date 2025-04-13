struct Position {
    x: i64,
    y: i64
}

fn main() {
    println!("Testse1");

    let mut position = Position { x: 1, y: 2 };
    position1(position);
    position3(&mut position);
    position3(&mut position);
    position2(&position);
}

fn position1(pos: Position) {
    println!("Position1 -> {{ x: {}, y: {} }}", pos.x, pos.y);
}

fn position2(pos: &Position) {
    println!("Position2 -> {{ x: {}, y: {} }}", pos.x, pos.y);
}

fn position3(pos: &mut Position) {
    pos.x = 21;
    println!("Position3 -> {{ x: {}, y: {} }}", pos.x, pos.y);
}