use crate::style::{Styles, Style};
use serde_json::Value;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Element<'a> {
    pub path: String,
    pub indices: Vec<usize>,
    pub value: &'a Value,
}

#[derive(Debug)]
pub struct Iter<'a> {
    style: Style<'a>,
    items: VecDeque<Element<'a>>,
}

impl<'a> Iter<'a> {
    pub fn new(json: &'a Value) -> Self {
        let mut queue = VecDeque::new();
        queue.push_back(Element {
            path: String::from(""),
            indices: Vec::new(),
            value: json,
        });

        Self {
            items: queue,
            style: Style::from(&Styles::SquareBrackets),
        }
    }

    pub fn use_style(mut self, style: &'a Styles) -> Self {
        self.style = Style::from(style);
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
        if let Some(el) = self.items.pop_front() {
            match el.value {
                Value::Object(obj) => {
                    for (key, val) in obj.iter().rev() {
                        let new_path = format!(
                            "{}{}{}{}",
                            el.path,
                            self.style.object_key_prefix,
                            key,
                            self.style.object_key_suffix,
                        );

                        self.items.push_front(Element {
                            path: new_path,
                            indices: el.indices.clone(),
                            value: val,
                        });
                    }
                    if !self.style.skip_parents {
                        return Some(el);
                    }
                }
                Value::Array(arr) => {
                    for (index, val) in arr.iter().enumerate().rev() {
                        let new_path = if self.style.indices_in_path {
                            format!(
                                "{}{}{}{}",
                                el.path,
                                self.style.array_key_prefix,
                                index,
                                self.style.array_key_suffix,
                            )
                        } else {
                            format!(
                                "{}{}{}",
                                el.path,
                                self.style.array_key_prefix,
                                self.style.array_key_suffix,
                            )
                        };

                        let mut indices_vec = el.indices.to_vec();
                        indices_vec.push(index);

                        self.items.push_front(Element {
                            path: new_path,
                            indices: indices_vec,
                            value: val,
                        });
                    }
                    if !self.style.skip_parents {
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
    fn test() {
        let val = json!({
            "a": {"b": 2},
        });

        for el in Iter::new(&val) {
            println!("$ {}: {}", el.path, el.value);
            /*
                $ : {"a":{"b":2}}
                $ ["a"]: {"b":2}
                $ ["a"]["b"]: 2
            */
        }

        for el in Iter::new(&val).use_style(&Styles::CommonJs) {
            println!("$ {}: {}", el.path, el.value);
            /*
                $ : {"a":{"b":2}}
                $ .a: {"b":2}
                $ .a.b: 2
            */
        }

        for el in Iter::new(&val).use_style(&Styles::PostgresJson) {
            println!("$ {}: {}", el.path, el.value);
            /*
                $ : {"a":{"b":2}}
                $ ->'a': {"b":2}
                $ ->'a'->'b': 2
            */
        }
    }

    #[test]
    fn test_1() {
        let val = json!({
            "a": {"x": [1,2,3]},
            "b": {"y": 2},
            "c": {"z": false},
        });
        let jkpi = Iter::new(&val);

        let list: Vec<_> = jkpi.collect();
        println!("jkpi: {} {:#?}", list.len(), list);
        assert_eq!(list.len(), 10);
    }

    #[test]
    fn test_one() {
        let val = json!({
            "a": {"x": [1,2,3]},
            "b": {"y": 2},
            "c": {"z": false},
        });
        let jkpi = Iter::new(&val).use_style(&Styles::CommonJs);

        let list: Vec<_> = jkpi.collect();
        println!("jkpi: {} {:#?}", list.len(), list);
        assert_eq!(list.len(), 10);
    }

    #[test]
    fn test_two() {
        let val = json!({
            "a": {"x": [1,2,3]},
            "b": {"y": 2},
            "c": {"z": true},
        });
        let jkpi = Iter::new(&val).use_style(&Styles::PostgresJson);

        let list: Vec<_> = jkpi.collect();
        println!("jkpi: {} {:#?}", list.len(), list);
        assert_eq!(list.len(), 10);
    }

    #[test]
    fn test_three() {
        let val = json!(["hello"]);
        let jkpi = Iter::new(&val);

        let list: Vec<_> = jkpi.collect();
        println!("jkpi: {} {:#?}", list.len(), list);
        assert_eq!(list.len(), 2);
    }


    #[test]
    fn test_four() {
        let val = json!({"msg": "hello"});
        let jkpi = Iter::new(&val);

        let list: Vec<_> = jkpi.collect();
        println!("jkpi: {} {:#?}", list.len(), list);
        assert_eq!(list.len(), 2);
        assert_eq!(list[1].path, "[\"msg\"]");
        assert_eq!(list[1].value.as_str(), json!("hello").as_str());
        // });
    }
}
