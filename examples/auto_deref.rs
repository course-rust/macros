use std::ops::Deref;

use macros::AutoDeref;

#[allow(unused)]
#[derive(AutoDeref)]
#[auto_deref(mutable = true, field = "inner")]
pub struct RespBulkString {
    inner: String,
    nothing: (),
}
fn main() {
    let resp = RespBulkString {
        inner: "Hello, world!".to_string(),
        nothing: (),
    };
    println!("{}", Deref::deref(&resp)); // prints "Hello, world!"
}
