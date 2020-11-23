use crate::style::{PresetStyle, Style};
use serde_json::Value;
use std::collections::VecDeque;

/// Single element struct containing the path, set of array indices, and json value
#[derive(Debug, PartialEq)]
pub struct Element<'a> {
    /// The full path from the base of a json structure to the value contained in the `Element`
    pub path: String,
    /// The full set of _array_ indices in the path, useful for grouping sets of `Element` structs to the same array element
    pub indices: Vec<usize>,
    /// The `serde_json::Value` of described by the path
    pub value: &'a Value,
}

/// Iteration strict containing a queue of elements that still need to be yielded along with a style object
#[derive(Debug)]
pub struct Iter<'a> {
    style: Style<'a>,
    items: VecDeque<Element<'a>>,
}

/// Named `Iter` internally, but `Iterator` externally
impl<'a> Iter<'a> {
    /// Create a new json keypath iterator
    ///
    /// Example:
    /// ```rust
    /// use serde_json::json;
    /// use json_keypath_iter::{Iterator, Element};
    ///
    /// let value = json!({"a": [1, 2]});
    /// let iter = Iterator::new(&value);
    /// let items: Vec<_> = iter.collect();
    ///
    /// assert_eq!(items[0], Element { path: "[\"a\"][0]".into(), indices: vec![0], value: &json!(1), });
    /// assert_eq!(items[1], Element { path: "[\"a\"][1]".into(), indices: vec![1], value: &json!(2), });
    /// ```
    pub fn new(json: &'a Value) -> Self {
        let mut queue = VecDeque::new();
        queue.push_back(Element {
            path: String::from(""),
            indices: Vec::new(),
            value: json,
        });

        Self {
            items: queue,
            style: PresetStyle::SquareBrackets.into(),
        }
    }

    /// Optionally used to set a custom style for the path in elements
    ///
    /// Example:
    /// ```rust
    /// use serde_json::json;
    /// use json_keypath_iter::{Style, PresetStyle, Iterator, Element};
    ///
    /// let style: Style = PresetStyle::CommonJs.into();
    /// let value = json!({"x42": [true, [null, "Hello there."]]});
    /// let iter = Iterator::new(&value).use_style(style);
    /// let items: Vec<_> = iter.collect();
    ///
    /// assert_eq!(items[0], Element { path: ".x42[0]".into(), indices: vec![0], value: &json!(true), });
    /// assert_eq!(items[1], Element { path: ".x42[1][0]".into(), indices: vec![1, 0], value: &json!(null), });
    /// assert_eq!(items[2], Element { path: ".x42[1][1]".into(), indices: vec![1, 1], value: &json!("Hello there."), });
    /// ```
    pub fn use_style(mut self, style: Style<'a>) -> Self {
        self.style = style;
        self
    }
}

impl<'a> From<&'a Value> for Iter<'a> {
    fn from(item: &'a Value) -> Iter<'a> {
        Iter::new(item)
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Element<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        'items: while let Some(el) = self.items.pop_front() {
            match el.value {
                Value::Object(obj) => {
                    for (key, val) in obj.iter().rev() {
                        self.items.push_front(Element {
                            path: self.style.object_format(&el.path, key),
                            indices: el.indices.clone(),
                            value: val,
                        });
                    }

                    match self.style.should_skip_object_parents() {
                        true => continue 'items,
                        false => return Some(el),
                    };
                }
                Value::Array(arr) => {
                    for (index, val) in arr.iter().enumerate().rev() {
                        let mut indices_vec = el.indices.to_vec();
                        indices_vec.push(index);

                        self.items.push_front(Element {
                            path: self.style.array_format(&el.path, index),
                            indices: indices_vec,
                            value: val,
                        });
                    }

                    match self.style.should_skip_array_parents() {
                        true => continue 'items,
                        false => return Some(el),
                    };
                }
                _ => return Some(el),
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::StyleBuilder;
    use serde_json::json;

    #[test]
    fn null_to_iter() {
        let value = json!(null);
        let items: Vec<_> = Iter::new(&value).collect();

        assert_eq!(items.len(), 1);
        assert_eq!(
            items[0],
            Element {
                path: String::from(""),
                indices: Vec::new(),
                value: &Value::Null,
            }
        );
    }

    #[test]
    fn bool_to_iter() {
        let value = json!(true);
        let items: Vec<_> = Iter::new(&value).collect();

        assert_eq!(items.len(), 1);
        assert_eq!(
            items[0],
            Element {
                path: String::from(""),
                indices: Vec::new(),
                value: &Value::Bool(true),
            }
        );
    }

    #[test]
    fn number_to_iter() {
        let value = json!(42);
        let items: Vec<_> = Iter::new(&value).collect();

        assert_eq!(items.len(), 1);
        assert_eq!(
            items[0],
            Element {
                path: String::from(""),
                indices: Vec::new(),
                value: &Value::Number(42.into()),
            }
        );
    }

    #[test]
    fn string_to_iter() {
        let value = json!("Hello there!");
        let items: Vec<_> = Iter::new(&value).collect();

        assert_eq!(items.len(), 1);
        assert_eq!(
            items[0],
            Element {
                path: String::from(""),
                indices: Vec::new(),
                value: &Value::String("Hello there!".into()),
            }
        );
    }

    #[test]
    fn array_to_iter() {
        let value = json!([null, null]);
        let style = StyleBuilder::new().include_array_parents().build();
        let items: Vec<_> = Iter::new(&value).use_style(style).collect();

        assert_eq!(items.len(), 3);
        assert_eq!(
            items[0],
            Element {
                path: String::from(""),
                indices: Vec::new(),
                value: &Value::Array(vec![Value::Null, Value::Null]),
            }
        );
    }

    #[test]
    fn object_to_iter() {
        let value = json!({ "a": true, "b": false });
        let style = StyleBuilder::new().include_object_parents().build();
        let items: Vec<_> = Iter::new(&value).use_style(style).collect();

        assert_eq!(items.len(), 3);
        assert_eq!(
            items[0],
            Element {
                path: String::from(""),
                indices: Vec::new(),
                value: &json!({ "a": true, "b": false }),
            }
        );
    }

    #[test]
    fn can_skip_parents() {
        let value = json!({
            "first": [1, 2, 3],
            "middle": true,
            "last": ["a", "b", "c"],
        });
        let style = StyleBuilder::new()
            .skip_object_parents()
            .skip_array_parents()
            .build();
        let items: Vec<_> = Iter::new(&value).use_style(style).collect();

        assert_eq!(items.len(), 7);
        assert_eq!(
            items[2],
            Element {
                path: String::from("[\"first\"][2]"),
                indices: vec![2],
                value: &Value::Number(3.into()),
            }
        );
        assert_eq!(
            items[5],
            Element {
                path: String::from("[\"last\"][2]"),
                indices: vec![2],
                value: &Value::String("c".into()),
            }
        );
    }

    #[test]
    fn custom_style_on_iter() {
        let value = json!({
            "first": [1, 2, 3],
        });
        let style = StyleBuilder::new()
            .object_key_prefix("!")
            .object_key_suffix("@")
            .show_object_keys_in_path()
            .include_object_parents()
            .array_key_prefix("#")
            .array_key_suffix("$")
            .hide_array_keys_in_path()
            .include_array_parents()
            .build();
        let items: Vec<_> = Iter::new(&value).use_style(style).collect();

        assert_eq!(items.len(), 5);
        assert_eq!(
            items[3],
            Element {
                path: String::from("!first@#$"),
                indices: vec![1],
                value: &Value::Number(2.into()),
            }
        );
    }

    #[test]
    fn complex_format_on_iter() {
        let value = json!({
            "first": [1, 2, 3],
            "middle": true,
            "last": ["a", "b", "c"],
        });
        let style = StyleBuilder::new()
            .include_object_parents()
            .include_array_parents()
            .build();
        let items: Vec<_> = Iter::new(&value).use_style(style).collect();

        assert_eq!(items.len(), 10);
        assert_eq!(
            items[2],
            Element {
                path: String::from("[\"first\"][0]"),
                indices: vec![0],
                value: &Value::Number(1.into()),
            }
        );
        assert_eq!(
            items[5],
            Element {
                path: String::from("[\"last\"]"),
                indices: Vec::new(),
                value: &Value::Array(vec!["a".into(), "b".into(), "c".into()]),
            }
        );
        assert_eq!(
            items[8],
            Element {
                path: String::from("[\"last\"][2]"),
                indices: vec![2],
                value: &Value::String("c".into()),
            }
        );

        // interesting note that "middle" is sorted alphabetically to the last object entry by json!()
        assert_eq!(
            items[9],
            Element {
                path: String::from("[\"middle\"]"),
                indices: Vec::new(),
                value: &Value::Bool(true),
            }
        );
    }

    #[test]
    fn in_a_for_loop() {
        let value = json!({
            "first": [1, 2, 3],
            "middle": true,
            "last": ["a", "b", "c"],
        });

        let mut collection = Vec::new();
        let style = StyleBuilder::new()
            .include_object_parents()
            .include_array_parents()
            .build();
        for item in Iter::new(&value).use_style(style) {
            collection.push(item);
        }

        assert_eq!(collection.len(), 10);
        assert_eq!(
            collection[2],
            Element {
                path: String::from("[\"first\"][0]"),
                indices: vec![0],
                value: &Value::Number(1.into()),
            }
        );
        assert_eq!(
            collection[5],
            Element {
                path: String::from("[\"last\"]"),
                indices: Vec::new(),
                value: &Value::Array(vec!["a".into(), "b".into(), "c".into()]),
            }
        );
        assert_eq!(
            collection[8],
            Element {
                path: String::from("[\"last\"][2]"),
                indices: vec![2],
                value: &Value::String("c".into()),
            }
        );

        // interesting note that "middle" is sorted alphabetically to the last object entry by json!()
        assert_eq!(
            collection[9],
            Element {
                path: String::from("[\"middle\"]"),
                indices: Vec::new(),
                value: &Value::Bool(true),
            }
        );
    }
}
