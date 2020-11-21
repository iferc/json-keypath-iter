pub mod builder;

pub enum Styles<'a> {
    SquareBrackets,
    CommonJs,
    PostgresJson,
    Custom(Style<'a>),
}

#[derive(Debug)]
pub struct Style<'a> {
    pub object_key_prefix: &'a str,
    pub object_key_suffix: &'a str,
    pub array_key_prefix: &'a str,
    pub array_key_suffix: &'a str,
    pub indices_in_path: bool,
    pub skip_parents: bool,
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
}

impl<'a> From<&'a Styles<'a>> for Style<'a> {
    fn from(style: &'a Styles) -> Style<'a> {
        match style {
            Styles::SquareBrackets => {
                return Style {
                    object_key_prefix: "[\"",
                    object_key_suffix: "\"]",
                    array_key_prefix: "[",
                    array_key_suffix: "]",
                    indices_in_path: true,
                    skip_parents: false,
                }
            }
            Styles::CommonJs => {
                return Style {
                    object_key_prefix: ".",
                    object_key_suffix: "",
                    array_key_prefix: "[",
                    array_key_suffix: "]",
                    indices_in_path: false,
                    skip_parents: false,
                }
            }
            Styles::PostgresJson => {
                return Style {
                    object_key_prefix: "->'",
                    object_key_suffix: "'",
                    array_key_prefix: "->",
                    array_key_suffix: "",
                    indices_in_path: true,
                    skip_parents: false,
                }
            }
            Styles::Custom(style_details) => {
                return Style {
                    object_key_prefix: style_details.object_key_prefix,
                    object_key_suffix: style_details.object_key_suffix,
                    array_key_prefix: style_details.array_key_prefix,
                    array_key_suffix: style_details.array_key_suffix,
                    indices_in_path: style_details.indices_in_path,
                    skip_parents: style_details.skip_parents,
                }
            }
        }
    }
}
