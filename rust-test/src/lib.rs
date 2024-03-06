use serde::Serialize;
use rlp::RlpStream;

#[derive(Serialize)]
struct Foo {
    foo: usize,
}

pub fn add(left: usize, right: usize) {
    let _ = RlpStream::default();
    println!("{}", serde_json::to_string(&Foo {foo: left + right}).unwrap());
}
