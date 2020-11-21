use json_keypath_iter::{Element, Iter, StyleBuilder};
use serde_json::{json, Value};

fn main() {
    let value = json!({
        "first": [1, 2, 3],
        "middle": true,
        "last": ["a", "b", "c"],
    });
    let style = StyleBuilder::new()
        .object_key_prefix("!")
        .object_key_suffix("@")
        .array_key_prefix("#")
        .array_key_suffix("$")
        .hide_indices_in_path()
        .skip_parents()
        .build()
        .unwrap();
    let items: Vec<_> = Iter::new(&value).use_style(style).collect();

    assert_eq!(items.len(), 7);
    assert_eq!(
        items[2],
        Element {
            path: String::from("!first@#$"),
            indices: vec![2],
            value: &Value::Number(3.into()),
        }
    );
    assert_eq!(
        items[5],
        Element {
            path: String::from("!last@#$"),
            indices: vec![2],
            value: &Value::String("c".into()),
        }
    );

    println!("Success!");
}
