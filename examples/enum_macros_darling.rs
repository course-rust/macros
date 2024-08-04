use macros::EnumFromDarling;

fn main() {
    let up: Direction = DirectionUp::new(10).into();
    let left: Direction = 20.into();
    println!("{:?}", up);
    println!("{:?}", left);
}

#[allow(unused)]
#[derive(Debug, EnumFromDarling)]
enum Direction {
    Up(DirectionUp),
    Down,
    Left(u32),
    Right { a: u32 },
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionUp {
    speed: i32,
}

impl DirectionUp {
    fn new(speed: i32) -> DirectionUp {
        DirectionUp { speed }
    }
}
