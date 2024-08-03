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

// my_ready!  =>  Poll::Ready / Poll::Pending
#[macro_export]
macro_rules! my_ready {
    ($e:expr) => {
        match $e {
            std::task::Poll::Ready(v) => std::task::Poll::Ready(v),
            std::task::Poll::Pending => return std::task::Poll::Pending,
        }
    };
}
