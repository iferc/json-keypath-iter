use super::*;

pub enum PresetStyle {
    SquareBrackets,
    CommonJs,
    PostgresJson,
}

impl<'a> From<&'a PresetStyle> for Style<'a> {
    fn from(style: &'a PresetStyle) -> Style<'a> {
        let builder: StyleBuilder<'a> = style.into();
        builder.build()
    }
}

impl<'a> From<&'a PresetStyle> for StyleBuilder<'a> {
    fn from(style: &'a PresetStyle) -> StyleBuilder<'a> {
        match style {
            PresetStyle::SquareBrackets => {
                return StyleBuilder::new()
                    .object_key_prefix("[\"")
                    .object_key_suffix("\"]")
                    .show_object_keys_in_path()
                    .skip_object_parents()
                    .array_key_prefix("[")
                    .array_key_suffix("]")
                    .show_array_keys_in_path()
                    .skip_array_parents();
            }
            PresetStyle::CommonJs => {
                return StyleBuilder::new()
                    .object_key_prefix(".")
                    .object_key_suffix("")
                    .show_object_keys_in_path()
                    .skip_object_parents()
                    .array_key_prefix("[")
                    .array_key_suffix("]")
                    .show_array_keys_in_path()
                    .skip_array_parents();
            }
            PresetStyle::PostgresJson => {
                return StyleBuilder::new()
                    .object_key_prefix("->'")
                    .object_key_suffix("'")
                    .show_object_keys_in_path()
                    .skip_object_parents()
                    .array_key_prefix("->")
                    .array_key_suffix("")
                    .show_array_keys_in_path()
                    .skip_array_parents();
            }
        }
    }
}
