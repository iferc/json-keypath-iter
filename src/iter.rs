use serde_json::{Value};
use std::collections::VecDeque;

pub enum JsonKeyPathStyle {
    SquareBrackets,
    CommonJs, // unsure of this name
    PostgresJson,
}

#[derive(Debug)]
pub struct JsonKeyPathElement<'a> {
    pub path: String,
    pub indices: Vec<usize>,
    pub value: &'a Value,
}

pub struct JsonKeyPathIter<'a> {
    object_key_prefix: &'a str,
    object_key_suffix: &'a str,
    array_key_prefix: &'a str,
    array_key_suffix: &'a str,
    indices_in_path: bool,
    /// whether to use callback when element has children
    skip_parents: bool,

    items: VecDeque<JsonKeyPathElement<'a>>,
}

impl<'a> JsonKeyPathIter<'a> {
    pub fn new(
        json: &'a Value,
    ) -> Self {
        let mut queue = VecDeque::new();
        queue.push_back(JsonKeyPathElement {
            path: String::from(""),
            indices: Vec::new(),
            value: json,
        });

        Self {
            object_key_prefix: "[\"",
            object_key_suffix: "\"]",
            array_key_prefix: "[",
            array_key_suffix: "]",
            indices_in_path: true,
            skip_parents: false,
            items: queue,
        }
    }

    pub fn new_with_style(
        object_key_prefix: &'a str,
        object_key_suffix: &'a str,
        array_key_prefix: &'a str,
        array_key_suffix: &'a str,
        indices_in_path: bool,
        skip_parents: bool,
        json: &'a Value,
    ) -> Self {
        let mut queue = VecDeque::new();
        queue.push_back(JsonKeyPathElement {
            path: String::from(""),
            indices: Vec::new(),
            value: json,
        });

        Self {
            object_key_prefix,
            object_key_suffix,
            array_key_prefix,
            array_key_suffix,
            indices_in_path,
            skip_parents,
            items: queue,
        }
    }

    pub fn use_style(mut self, style: JsonKeyPathStyle) -> Self {
        match style {
            JsonKeyPathStyle::SquareBrackets => {
                self.object_key_prefix = "[\"";
                self.object_key_suffix = "\"]";
                self.array_key_prefix = "[";
                self.array_key_suffix = "]";
                self.indices_in_path = true;
                self.skip_parents = false;
            }
            JsonKeyPathStyle::CommonJs => {
                self.object_key_prefix = ".";
                self.object_key_suffix = "";
                self.array_key_prefix = "[";
                self.array_key_suffix = "]";
                self.indices_in_path = true;
                self.skip_parents = false;
            }
            JsonKeyPathStyle::PostgresJson => {
                self.object_key_prefix = "->'";
                self.object_key_suffix = "'";
                self.array_key_prefix = "->";
                self.array_key_suffix = "";
                self.indices_in_path = true;
                self.skip_parents = false;
            }
        }
        self
    }
}

impl<'a> From<&'a Value> for JsonKeyPathIter<'a> {
    fn from(item: &'a Value) -> JsonKeyPathIter<'a> {
        JsonKeyPathIter::new(item)
    }
}

impl<'a> Iterator for JsonKeyPathIter<'a> {
    type Item = JsonKeyPathElement<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(el) = self.items.pop_front() {
            match el.value {
                Value::Object(obj) => {
                    for (key, val) in obj.iter().rev() {
                        let new_path = format!(
                            "{}{}{}{}",
                            el.path,
                            self.object_key_prefix,
                            key,
                            self.object_key_suffix,
                        );

                        self.items.push_front(JsonKeyPathElement {
                            path: new_path,
                            indices: el.indices.clone(),
                            value: val,
                        });
                    }
                    if !self.skip_parents {
                        return Some(el);
                    }
                }
                Value::Array(arr) => {
                    for (index, val) in arr.iter().enumerate().rev() {
                        let new_path = if self.indices_in_path {
                            format!(
                                "{}{}{}{}",
                                el.path,
                                self.array_key_prefix,
                                index,
                                self.array_key_suffix,
                            )
                        } else {
                            format!(
                                "{}{}{}",
                                el.path,
                                self.array_key_prefix,
                                self.array_key_suffix,
                            )
                        };

                        let mut indices_vec = el.indices.to_vec();
                        indices_vec.push(index);

                        self.items.push_front(JsonKeyPathElement {
                            path: new_path,
                            indices: indices_vec,
                            value: val,
                        });
                    }
                    if !self.skip_parents {
                        return Some(el);
                    }
                }
                _ => return Some(el)
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_1() {
        let val = json!({
            "a": {"x": [1,2,3]},
            "b": {"y": 2},
            "c": {"z": false},
        });

        let jkpi = JsonKeyPathIter::new(&val);

        println!("\nbeginning:");
        for el in jkpi {
            println!("  el.1: {:?}", el);
        }
        println!("done;\n");
    }

    #[test]
    fn test_one() {
        let val = json!({
            "a": {"x": [1,2,3]},
            "b": {"y": 2},
            "c": {"z": false},
        });

        let mut queue = VecDeque::new();
        queue.push_back(JsonKeyPathElement {
            path: String::from("BASE"),
            indices: Vec::new(),
            value: &val,
        });

        let jkpi = JsonKeyPathIter {
            object_key_prefix: ".",
            object_key_suffix: "",
            array_key_prefix: "[",
            array_key_suffix: "]",
            indices_in_path: true,
            skip_parents: false,

            items: queue,
        };

        println!("\nbeginning:");
        for el in jkpi {
            println!("  el1: {:?}", el);
        }
        println!("done;\n");
    }

    #[test]
    fn test_two() {
        let val = json!({
            "a": {"x": [1,2,3]},
            "b": {"y": 2},
            "c": {"z": true},
        });

        let mut queue = VecDeque::new();
        queue.push_back(JsonKeyPathElement {
            path: String::from("BASE"),
            indices: Vec::new(),
            value: &val,
        });

        let jkpi = JsonKeyPathIter {
            object_key_prefix: "->'",
            object_key_suffix: "'",
            array_key_prefix: "->",
            array_key_suffix: "",
            indices_in_path: true,
            skip_parents: false,

            items: queue,
        };

        println!("\nbeginning:");
        for el in jkpi {
            println!("  el2: {:?}", el);
        }
        println!("done;\n");
    }

    #[test]
    fn test_three() {
        let val = json!(["hello"]);

        let mut queue = VecDeque::new();
        queue.push_back(JsonKeyPathElement {
            path: String::from("BASE"),
            indices: Vec::new(),
            value: &val,
        });

        let jkpi = JsonKeyPathIter {
            object_key_prefix: ".",
            object_key_suffix: "",
            array_key_prefix: "[",
            array_key_suffix: "]",
            indices_in_path: false,
            skip_parents: false,

            items: queue,
        };

        println!("\nbeginning:");
        for el in jkpi {
            println!("  el3: {:?}", el);
        }
        println!("done;\n");
    }


    #[test]
    fn test_four() {
        let val = json!({"msg": "hello"});

        let mut queue = VecDeque::new();
        queue.push_back(JsonKeyPathElement {
            path: String::from("BASE"),
            indices: Vec::new(),
            value: &val,
        });

        let jkpi = JsonKeyPathIter {
            object_key_prefix: ".",
            object_key_suffix: "",
            array_key_prefix: "[",
            array_key_suffix: "]",
            indices_in_path: true,
            skip_parents: false,

            items: queue,
        };

        println!("\nbeginning:");
        for el in jkpi {
            println!("  el4: {:?}", el);
        }
        println!("done;\n");
    }
}
