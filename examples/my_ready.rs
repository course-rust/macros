use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

#[tokio::main]
async fn main() {
    let fut = MyFut::new(42);
    println!("Final result: {:?}", fut.await);
}

#[allow(unused)]
fn poll_fut(ctx: &mut Context<'_>) -> Poll<usize> {
    let mut fut = MyFut::new(42);
    // let mut fut = future::ready(42);
    let fut = Pin::new(&mut fut);
    my_ready!(fut.poll(ctx))
}

struct MyFut {
    polled: bool,
    v: usize,
}
impl MyFut {
    fn new(v: usize) -> MyFut {
        MyFut { polled: false, v }
    }
}
impl Future for MyFut {
    type Output = usize;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.polled {
            Poll::Ready(self.v)
        } else {
            self.polled = true;
            // wake up the waker
            cx.waker().wake_by_ref();
            // Poll::Pending until the future is ready to produce a value
            Poll::Pending
        }
    }
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
