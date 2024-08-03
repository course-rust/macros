use macros::EnumFrom;

fn main() {
    let up: Direction = DirectionUp::new(10).into();
    let left: Direction = 20.into();
    println!("{:?}", up);
    println!("{:?}", left);
}

#[allow(unused)]
#[derive(Debug, EnumFrom)]
enum Direction {
    Up(DirectionUp),
    Down,
    Left(u32),
    Right { a: u32 },
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionUp {
    speed: u32,
}

impl DirectionUp {
    fn new(speed: u32) -> DirectionUp {
        DirectionUp { speed }
    }
}
