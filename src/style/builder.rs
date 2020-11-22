use super::*;

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

    pub fn default_object_key_prefix(mut self) -> Self {
        self.object_key_prefix = None;
        self
    }
    pub fn object_key_prefix(mut self, value: &'a str) -> Self {
        self.object_key_prefix = Some(value);
        self
    }

    pub fn default_object_key_suffix(mut self) -> Self {
        self.object_key_suffix = None;
        self
    }
    pub fn object_key_suffix(mut self, value: &'a str) -> Self {
        self.object_key_suffix = Some(value);
        self
    }

    pub fn default_object_keys_in_path(mut self) -> Self {
        self.object_keys_in_path = None;
        self
    }
    pub fn show_object_keys_in_path(mut self) -> Self {
        self.object_keys_in_path = Some(true);
        self
    }
    pub fn hide_object_keys_in_path(mut self) -> Self {
        self.object_keys_in_path = Some(false);
        self
    }

    pub fn default_object_parents(mut self) -> Self {
        self.skip_object_parents = None;
        self
    }
    pub fn skip_object_parents(mut self) -> Self {
        self.skip_object_parents = Some(true);
        self
    }
    pub fn include_object_parents(mut self) -> Self {
        self.skip_object_parents = Some(false);
        self
    }

    pub fn default_array_key_prefix(mut self) -> Self {
        self.array_key_prefix = None;
        self
    }
    pub fn array_key_prefix(mut self, value: &'a str) -> Self {
        self.array_key_prefix = Some(value);
        self
    }

    pub fn default_array_key_suffix(mut self) -> Self {
        self.array_key_suffix = None;
        self
    }
    pub fn array_key_suffix(mut self, value: &'a str) -> Self {
        self.array_key_suffix = Some(value);
        self
    }

    pub fn default_array_keys_in_path(mut self) -> Self {
        self.array_keys_in_path = None;
        self
    }
    pub fn show_array_keys_in_path(mut self) -> Self {
        self.array_keys_in_path = Some(true);
        self
    }
    pub fn hide_array_keys_in_path(mut self) -> Self {
        self.array_keys_in_path = Some(false);
        self
    }

    pub fn default_array_parents(mut self) -> Self {
        self.skip_array_parents = None;
        self
    }
    pub fn skip_array_parents(mut self) -> Self {
        self.skip_array_parents = Some(true);
        self
    }
    pub fn include_array_parents(mut self) -> Self {
        self.skip_array_parents = Some(false);
        self
    }

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
