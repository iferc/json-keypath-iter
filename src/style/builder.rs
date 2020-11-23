use super::*;

/// Builder to customise path styling
pub struct StyleBuilder<'a> {
    object_key_prefix: Option<&'a str>,
    object_key_suffix: Option<&'a str>,
    object_keys_in_path: Option<bool>,
    skip_object_parents: Option<bool>,
    array_key_prefix: Option<&'a str>,
    array_key_suffix: Option<&'a str>,
    array_keys_in_path: Option<bool>,
    skip_array_parents: Option<bool>,
}

impl<'a> StyleBuilder<'a> {
    pub fn new() -> Self {
        StyleBuilder {
            object_key_prefix: None,
            object_key_suffix: None,
            object_keys_in_path: None,
            skip_object_parents: None,
            array_key_prefix: None,
            array_key_suffix: None,
            array_keys_in_path: None,
            skip_array_parents: None,
        }
    }

    /// Clears the currently specified object key prefix value
    pub fn default_object_key_prefix(mut self) -> Self {
        self.object_key_prefix = None;
        self
    }
    /// Sets the object key prefix value
    /// ```rust
    /// use serde_json::json;
    /// use json_keypath_iter::{Style, StyleBuilder, Iterator, Element};
    ///
    /// let style: Style = StyleBuilder::new()
    ///     .object_key_prefix(">>>")
    ///     .build();
    /// let value = json!({"apple": [1, true, "three"]});
    /// let iter = Iterator::new(&value).use_style(style);
    /// let items: Vec<_> = iter.collect();
    ///
    /// assert_eq!(items[0], Element { path: ">>>apple\"][0]".into(), indices: vec![0], value: &json!(1), });
    /// ```
    pub fn object_key_prefix(mut self, value: &'a str) -> Self {
        self.object_key_prefix = Some(value);
        self
    }

    /// Clears the currently specified object key suffix value
    pub fn default_object_key_suffix(mut self) -> Self {
        self.object_key_suffix = None;
        self
    }
    /// Sets the object key suffix value
    /// ```rust
    /// use serde_json::json;
    /// use json_keypath_iter::{Style, StyleBuilder, Iterator, Element};
    ///
    /// let style: Style = StyleBuilder::new()
    ///     .object_key_suffix("$$$")
    ///     .build();
    /// let value = json!({"apple": [1, true, "three"]});
    /// let iter = Iterator::new(&value).use_style(style);
    /// let items: Vec<_> = iter.collect();
    ///
    /// assert_eq!(items[0], Element { path: "[\"apple$$$[0]".into(), indices: vec![0], value: &json!(1), });
    /// ```
    pub fn object_key_suffix(mut self, value: &'a str) -> Self {
        self.object_key_suffix = Some(value);
        self
    }

    /// Clears whether to show or hide object key values in the Element path
    pub fn default_object_keys_in_path(mut self) -> Self {
        self.object_keys_in_path = None;
        self
    }
    /// Sets the object key values to be visible in the Element path
    /// ```rust
    /// use serde_json::json;
    /// use json_keypath_iter::{Style, StyleBuilder, Iterator, Element};
    ///
    /// let style: Style = StyleBuilder::new()
    ///     .show_object_keys_in_path()
    ///     .build();
    /// let value = json!({"apple": [1, true, "three"]});
    /// let iter = Iterator::new(&value).use_style(style);
    /// let items: Vec<_> = iter.collect();
    ///
    /// assert_eq!(items[0], Element { path: "[\"apple\"][0]".into(), indices: vec![0], value: &json!(1), });
    /// ```
    pub fn show_object_keys_in_path(mut self) -> Self {
        self.object_keys_in_path = Some(true);
        self
    }
    /// Sets the object key values to be hidden in the Element path
    /// ```rust
    /// use serde_json::json;
    /// use json_keypath_iter::{Style, StyleBuilder, Iterator, Element};
    ///
    /// let style: Style = StyleBuilder::new()
    ///     .hide_object_keys_in_path()
    ///     .build();
    /// let value = json!({"apple": [1, true, "three"]});
    /// let iter = Iterator::new(&value).use_style(style);
    /// let items: Vec<_> = iter.collect();
    ///
    /// assert_eq!(items[0], Element { path: "[\"\"][0]".into(), indices: vec![0], value: &json!(1), });
    /// ```
    pub fn hide_object_keys_in_path(mut self) -> Self {
        self.object_keys_in_path = Some(false);
        self
    }

    /// Clears whether to skip or include values that are objects in the set of yielded values
    pub fn default_object_parents(mut self) -> Self {
        self.skip_object_parents = None;
        self
    }
    /// Sets values that are objects to be yielded by the Iterator
    /// ```rust
    /// use serde_json::json;
    /// use json_keypath_iter::{Style, StyleBuilder, Iterator, Element};
    ///
    /// let style: Style = StyleBuilder::new()
    ///     .skip_object_parents()
    ///     .build();
    /// let value = json!({"apple": [1, true, "three"]});
    /// let iter = Iterator::new(&value).use_style(style);
    /// let items: Vec<_> = iter.collect();
    ///
    /// assert_eq!(items[0], Element { path: "[\"apple\"][0]".into(), indices: vec![0], value: &json!(1), });
    /// ```
    pub fn skip_object_parents(mut self) -> Self {
        self.skip_object_parents = Some(true);
        self
    }
    /// Prevents values that are objects from being yielded by the Iterator
    /// ```rust
    /// use serde_json::json;
    /// use json_keypath_iter::{Style, StyleBuilder, Iterator, Element};
    ///
    /// let style: Style = StyleBuilder::new()
    ///     .include_object_parents()
    ///     .build();
    /// let value = json!({"apple": [1, true, "three"]});
    /// let iter = Iterator::new(&value).use_style(style);
    /// let items: Vec<_> = iter.collect();
    ///
    /// assert_eq!(items[0], Element { path: "".into(), indices: vec![], value: &json!({"apple": [1, true, "three"]}), });
    /// ```
    pub fn include_object_parents(mut self) -> Self {
        self.skip_object_parents = Some(false);
        self
    }

    /// Clears the currently specified array_key_prefix value
    pub fn default_array_key_prefix(mut self) -> Self {
        self.array_key_prefix = None;
        self
    }
    /// Sets the array_key_prefix value
    /// ```rust
    /// use serde_json::json;
    /// use json_keypath_iter::{Style, StyleBuilder, Iterator, Element};
    ///
    /// let style: Style = StyleBuilder::new()
    ///     .array_key_prefix(":::")
    ///     .build();
    /// let value = json!({"apple": [1, true, "three"]});
    /// let iter = Iterator::new(&value).use_style(style);
    /// let items: Vec<_> = iter.collect();
    ///
    /// assert_eq!(items[0], Element { path: "[\"apple\"]:::0]".into(), indices: vec![0], value: &json!(1), });
    /// ```
    pub fn array_key_prefix(mut self, value: &'a str) -> Self {
        self.array_key_prefix = Some(value);
        self
    }

    /// Clears the currently specified array key suffix value
    pub fn default_array_key_suffix(mut self) -> Self {
        self.array_key_suffix = None;
        self
    }
    /// Sets the array key suffix value
    /// ```rust
    /// use serde_json::json;
    /// use json_keypath_iter::{Style, StyleBuilder, Iterator, Element};
    ///
    /// let style: Style = StyleBuilder::new()
    ///     .array_key_suffix("!!!")
    ///     .build();
    /// let value = json!({"apple": [1, true, "three"]});
    /// let iter = Iterator::new(&value).use_style(style);
    /// let items: Vec<_> = iter.collect();
    ///
    /// assert_eq!(items[0], Element { path: "[\"apple\"][0!!!".into(), indices: vec![0], value: &json!(1), });
    /// ```
    pub fn array_key_suffix(mut self, value: &'a str) -> Self {
        self.array_key_suffix = Some(value);
        self
    }

    /// Clears whether to show or hide array key values in the Element path
    pub fn default_array_keys_in_path(mut self) -> Self {
        self.array_keys_in_path = None;
        self
    }
    /// Sets the array key values to be visible in the Element path
    /// ```rust
    /// use serde_json::json;
    /// use json_keypath_iter::{Style, StyleBuilder, Iterator, Element};
    ///
    /// let style: Style = StyleBuilder::new()
    ///     .show_array_keys_in_path()
    ///     .build();
    /// let value = json!({"apple": [1, true, "three"]});
    /// let iter = Iterator::new(&value).use_style(style);
    /// let items: Vec<_> = iter.collect();
    ///
    /// assert_eq!(items[0], Element { path: "[\"apple\"][0]".into(), indices: vec![0], value: &json!(1), });
    /// ```
    pub fn show_array_keys_in_path(mut self) -> Self {
        self.array_keys_in_path = Some(true);
        self
    }
    /// Sets the array key values to be hidden in the Element path
    /// ```rust
    /// use serde_json::json;
    /// use json_keypath_iter::{Style, StyleBuilder, Iterator, Element};
    ///
    /// let style: Style = StyleBuilder::new()
    ///     .hide_array_keys_in_path()
    ///     .build();
    /// let value = json!({"apple": [1, true, "three"]});
    /// let iter = Iterator::new(&value).use_style(style);
    /// let items: Vec<_> = iter.collect();
    ///
    /// assert_eq!(items[0], Element { path: "[\"apple\"][]".into(), indices: vec![0], value: &json!(1), });
    /// ```
    pub fn hide_array_keys_in_path(mut self) -> Self {
        self.array_keys_in_path = Some(false);
        self
    }

    /// Clears whether to skip or include values that are arrays in the set of yielded values
    pub fn default_array_parents(mut self) -> Self {
        self.skip_array_parents = None;
        self
    }
    /// Sets values that are arrays to be yielded by the Iterator
    /// ```rust
    /// use serde_json::json;
    /// use json_keypath_iter::{Style, StyleBuilder, Iterator, Element};
    ///
    /// let style: Style = StyleBuilder::new()
    ///     .skip_array_parents()
    ///     .build();
    /// let value = json!({"apple": [1, true, "three"]});
    /// let iter = Iterator::new(&value).use_style(style);
    /// let items: Vec<_> = iter.collect();
    ///
    /// assert_eq!(items[0], Element { path: "[\"apple\"][0]".into(), indices: vec![0], value: &json!(1), });
    /// ```
    pub fn skip_array_parents(mut self) -> Self {
        self.skip_array_parents = Some(true);
        self
    }
    /// Prevents values that are arrays from being yielded by the Iterator
    /// ```rust
    /// use serde_json::json;
    /// use json_keypath_iter::{Style, StyleBuilder, Iterator, Element};
    ///
    /// let style: Style = StyleBuilder::new()
    ///     .include_array_parents()
    ///     .build();
    /// let value = json!({"apple": [1, true, "three"]});
    /// let iter = Iterator::new(&value).use_style(style);
    /// let items: Vec<_> = iter.collect();
    ///
    /// assert_eq!(items[0], Element { path: "[\"apple\"]".into(), indices: vec![], value: &json!([1, true, "three"]), });
    /// ```
    pub fn include_array_parents(mut self) -> Self {
        self.skip_array_parents = Some(false);
        self
    }

    /// Builds a value Style with defaults for any value not specified or previously cleared out
    pub fn build(&self) -> Style<'a> {
        Style {
            object_key_prefix: self.object_key_prefix.unwrap_or("[\""),
            object_key_suffix: self.object_key_suffix.unwrap_or("\"]"),
            object_keys_in_path: self.object_keys_in_path.unwrap_or(true),
            skip_object_parents: self.skip_object_parents.unwrap_or(true),
            array_key_prefix: self.array_key_prefix.unwrap_or("["),
            array_key_suffix: self.array_key_suffix.unwrap_or("]"),
            array_keys_in_path: self.array_keys_in_path.unwrap_or(true),
            skip_array_parents: self.skip_array_parents.unwrap_or(true),
        }
    }
}
