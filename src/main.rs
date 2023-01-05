use serde::Serialize;

extern crate serde_json;

#[derive(Serialize)]
struct Data {
    num: u128,
}

fn main() {
    println!("{}", serde_json::json!(Data { num: 1 }))
}
