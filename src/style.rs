pub enum JsonKeyPathStyles<'a> {
    SquareBrackets,
    CommonJs,
    PostgresJson,
    Custom(JsonKeyPathStyle<'a>),
}

#[derive(Debug)]
pub struct JsonKeyPathStyle<'a> {
    pub object_key_prefix: &'a str,
    pub object_key_suffix: &'a str,
    pub array_key_prefix: &'a str,
    pub array_key_suffix: &'a str,
    pub indices_in_path: bool,
    pub skip_parents: bool,
}

impl<'a> From<&'a JsonKeyPathStyles<'a>> for JsonKeyPathStyle<'a> {
    fn from(style: &'a JsonKeyPathStyles) -> JsonKeyPathStyle<'a> {
        match style {
            JsonKeyPathStyles::SquareBrackets => {
                return JsonKeyPathStyle {
                    object_key_prefix: "[\"",
                    object_key_suffix: "\"]",
                    array_key_prefix: "[",
                    array_key_suffix: "]",
                    indices_in_path: true,
                    skip_parents: false,
                }
            }
            JsonKeyPathStyles::CommonJs => {
                return JsonKeyPathStyle {
                    object_key_prefix: ".",
                    object_key_suffix: "",
                    array_key_prefix: "[",
                    array_key_suffix: "]",
                    indices_in_path: true,
                    skip_parents: false,
                }
            }
            JsonKeyPathStyles::PostgresJson => {
                return JsonKeyPathStyle {
                    object_key_prefix: "->'",
                    object_key_suffix: "'",
                    array_key_prefix: "->",
                    array_key_suffix: "",
                    indices_in_path: true,
                    skip_parents: false,
                }
            }
            JsonKeyPathStyles::Custom(style_details) => {
                return JsonKeyPathStyle {
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
