use tera::{Context, Tera};
use serde_json::{json, Value};

fn main() {

    let json = json!({
        "_somePrice": {
            "cash": 10,
            "period": 3
        },
    });

    let context = Context::from_value(json).unwrap();
    let result = Tera::one_off(include_str!("../trivial.txt"), &context, true);
    println!("{:#?}", result);
}
