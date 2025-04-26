

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Vector {
    x: i32,
    y: i32,
}


#[derive(Debug)]
struct Player {
    center: Point,
    radius: i8,
    velocity: Vector,
    acceleration: Vector,
}

#[derive(Debug)]
struct GameMap {
    height: i32,
    width: i32,
}    



fn main() {
    println!("yeet");

    let p1: Player = Player {
        center: Point { x: 10, y: 10 },
        radius: 5,
        velocity: Vector { x: 3, y: 1},
        acceleration: Vector { x: 1, y: -1},
    };

    println!("{:?}", p1);
}
