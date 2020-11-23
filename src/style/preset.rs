use super::*;

/// Included preset stylings
pub enum PresetStyle {
    /// This yields a path that looks like: `["some_key"][123]`
    ///
    /// The Iterator also yields only non-object and non-array values with this style
    SquareBrackets,
    /// This yields a path that looks like: `.some_key[123]`
    ///
    /// The Iterator also yields only non-object and non-array values with this style
    CommonJs,
    /// This yields a path that looks like: `->'some_key'->123`
    ///
    /// The Iterator also yields only non-object and non-array values with this style
    PostgresJson,
}

impl<'a> From<PresetStyle> for Style<'a> {
    fn from(style: PresetStyle) -> Style<'a> {
        let builder: StyleBuilder<'a> = style.into();
        builder.build()
    }
}

impl<'a> From<PresetStyle> for StyleBuilder<'a> {
    fn from(style: PresetStyle) -> StyleBuilder<'a> {
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
