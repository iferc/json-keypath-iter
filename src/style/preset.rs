use super::*;

pub enum PresetStyle<'a> {
    SquareBrackets,
    CommonJs,
    PostgresJson,
    Custom(Style<'a>),
}

impl<'a> From<&'a PresetStyle<'a>> for Style<'a> {
    fn from(style: &'a PresetStyle) -> Style<'a> {
        match style {
            PresetStyle::SquareBrackets => {
                return Style {
                    object_key_prefix: "[\"",
                    object_key_suffix: "\"]",
                    array_key_prefix: "[",
                    array_key_suffix: "]",
                    indices_in_path: true,
                    skip_parents: false,
                }
            }
            PresetStyle::CommonJs => {
                return Style {
                    object_key_prefix: ".",
                    object_key_suffix: "",
                    array_key_prefix: "[",
                    array_key_suffix: "]",
                    indices_in_path: false,
                    skip_parents: false,
                }
            }
            PresetStyle::PostgresJson => {
                return Style {
                    object_key_prefix: "->'",
                    object_key_suffix: "'",
                    array_key_prefix: "->",
                    array_key_suffix: "",
                    indices_in_path: true,
                    skip_parents: false,
                }
            }
            PresetStyle::Custom(style_details) => {
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
