mod builder;
mod preset;
pub use builder::StyleBuilder;
pub use preset::PresetStyle;

#[derive(Debug)]
pub struct Style<'a> {
    object_key_prefix: &'a str,
    object_key_suffix: &'a str,
    object_keys_in_path: bool,
    skip_object_parents: bool,
    array_key_prefix: &'a str,
    array_key_suffix: &'a str,
    array_keys_in_path: bool,
    skip_array_parents: bool,
}

impl<'a> Style<'a> {
    pub fn object_format(&self, base_path: &String, key: &String) -> String {
        if self.object_keys_in_path {
            format!(
                "{}{}{}{}",
                base_path, self.object_key_prefix, key, self.object_key_suffix,
            )
        } else {
            format!(
                "{}{}{}",
                base_path, self.object_key_prefix, self.object_key_suffix,
            )
        }
    }

    pub fn array_format(&self, base_path: &String, index: usize) -> String {
        if self.array_keys_in_path {
            format!(
                "{}{}{}{}",
                base_path, self.array_key_prefix, index, self.array_key_suffix,
            )
        } else {
            format!(
                "{}{}{}",
                base_path, self.array_key_prefix, self.array_key_suffix,
            )
        }
    }

    pub fn should_skip_object_parents(&self) -> bool {
        self.skip_object_parents
    }

    pub fn should_skip_array_parents(&self) -> bool {
        self.skip_array_parents
    }
}
