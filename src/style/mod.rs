mod builder;
mod preset;
pub use builder::StyleBuilder;
pub use preset::PresetStyle;

#[derive(Debug)]
pub struct Style<'a> {
    object_key_prefix: &'a str,
    object_key_suffix: &'a str,
    array_key_prefix: &'a str,
    array_key_suffix: &'a str,
    indices_in_path: bool,
    skip_parents: bool,
}

impl<'a> Style<'a> {
    pub fn object_format(&self, path: &String, key: &String) -> String {
        format!(
            "{}{}{}{}",
            path, self.object_key_prefix, key, self.object_key_suffix,
        )
    }

    pub fn array_format(&self, path: &String, index: usize) -> String {
        if self.indices_in_path {
            format!(
                "{}{}{}{}",
                path, self.array_key_prefix, index, self.array_key_suffix,
            )
        } else {
            format!("{}{}{}", path, self.array_key_prefix, self.array_key_suffix)
        }
    }

    pub fn skip_parents(&self) -> bool {
        self.skip_parents
    }
}
