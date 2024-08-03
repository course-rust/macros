fn main() {
    let v: Vec<i32> = my_vec![
        "1".parse().unwrap(),
        "2".parse().unwrap(),
        "3".parse().unwrap(),
        "4".parse().unwrap(),
        "5".parse().unwrap(),
        // "6".parse::<i32>() // Err: ParseIntError
    ];
    //let v = <[_]>::into_vec(Box::new([1, 2, 3, 4, 5]));
    //    let v: Vec<i32> = <[_]>::into_vec(Box::new([
    //        ("1".parse().unwrap()),
    //        ("2".parse().unwrap()),
    //        ("3".parse().unwrap()),
    //        ("4".parse().unwrap()),
    //        ("5".parse().unwrap()),
    //    ]));
    println!("{:?}", v);
}

// my_vec! = my_vec![1, 2, 3, 4, 5]; // Vec<i32>

#[macro_export]
macro_rules! my_vec {
    () => {
        Vec::new()
    };
    ($elem:expr; $n:expr) => {
        std::vec::from_elem($elem, $n)
    };
    ($($x:expr),+ $(,)?) => {
        {
            <[_]>::into_vec(Box::new([$($x),*]))
        }
    };
}
