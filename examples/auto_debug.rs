use macros::AutoDebug;

#[allow(unused)]
#[derive(AutoDebug)]
pub struct RespBulkString {
    inner: String,
    #[debug(skip = true)]
    nothing: (),
    hello: u32,
}

fn main() {
    let resp = RespBulkString {
        inner: "hello".to_string(),
        nothing: (),
        hello: 20,
    };
    println!("{:?}", resp);
}
