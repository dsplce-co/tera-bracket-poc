use tera::{Context, Tera};
use serde_json::{json, Value};

fn main() {

    let json = json!({
        "_someTime": {
            "time": 1652868838,
        },
    });

    let context = Context::from_value(json).unwrap();
    let result = Tera::one_off(include_str!("../timestamp.txt"), &context, true);
    println!("{:#?} ", result);
}
