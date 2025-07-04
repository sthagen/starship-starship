use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct VcshConfig<'a> {
    pub symbol: &'a str,
    pub style: &'a str,
    pub format: &'a str,
    pub disabled: bool,
}

impl Default for VcshConfig<'_> {
    fn default() -> Self {
        Self {
            symbol: "",
            style: "bold yellow",
            format: "vcsh [$symbol$repo]($style) ",
            disabled: false,
        }
    }
}
