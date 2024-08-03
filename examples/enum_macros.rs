use macros::EnumFrom;

fn main() {
    let up: Direction<u32> = DirectionUp::new(10).into();
    let left: Direction<u32> = 20.into();
    println!("{:?}", up);
    println!("{:?}", left);
}

#[allow(unused)]
#[derive(Debug, EnumFrom)]
enum Direction<T> {
    Up(DirectionUp<T>),
    Down,
    Left(u32),
    Right { a: u32 },
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionUp<T> {
    speed: T,
}

impl<T> DirectionUp<T> {
    fn new(speed: T) -> DirectionUp<T> {
        DirectionUp { speed }
    }
}
