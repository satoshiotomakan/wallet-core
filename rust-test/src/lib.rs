use serde::Serialize;

#[derive(Serialize)]
struct Foo {
    foo: usize,
}

pub fn add(left: usize, right: usize) {
    println!("{}", serde_json::to_string(&Foo {foo: left + right}).unwrap());
}
