use crate::Style;

pub struct StyleBuilder<'a>
{
    object_key_prefix: Option<&'a str>,
    object_key_suffix: Option<&'a str>,
    array_key_prefix: Option<&'a str>,
    array_key_suffix: Option<&'a str>,
    indices_in_path: bool,
    /// whether to use callback when element has children
    skip_parents: bool,
}

impl<'a> StyleBuilder<'a> {
    pub fn new() -> Self {
        StyleBuilder {
            object_key_prefix: None,
            object_key_suffix: None,
            array_key_prefix: None,
            array_key_suffix: None,
            indices_in_path: true,
            skip_parents: false,
        }
    }

    pub fn object_key_prefix(mut self, value: &'a str) -> Self {
        self.object_key_prefix = Some(value);
        self
    }

    pub fn object_key_suffix(mut self, value: &'a str) -> Self {
        self.object_key_suffix = Some(value);
        self
    }

    pub fn array_key_prefix(mut self, value: &'a str) -> Self {
        self.array_key_prefix = Some(value);
        self
    }

    pub fn array_key_suffix(mut self, value: &'a str) -> Self {
        self.array_key_suffix = Some(value);
        self
    }

    pub fn show_indices_in_path(mut self) -> Self {
        self.indices_in_path = true;
        self
    }

    pub fn hide_indices_in_path(mut self) -> Self {
        self.indices_in_path = false;
        self
    }

    pub fn skip_parents(mut self) -> Self {
        self.skip_parents = true;
        self
    }

    pub fn show_parents(mut self) -> Self {
        self.skip_parents = false;
        self
    }

    pub fn build(&self) -> Result<Style<'a>, &'static str> {
        Ok(Style {
            object_key_prefix: self.object_key_prefix.unwrap_or("[\""),
            object_key_suffix: self.object_key_suffix.unwrap_or("\"]"),
            array_key_prefix: self.array_key_prefix.unwrap_or("["),
            array_key_suffix: self.array_key_suffix.unwrap_or("]"),
            indices_in_path: self.indices_in_path,
            skip_parents: self.skip_parents,
        })
    }
}
