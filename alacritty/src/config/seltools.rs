use alacritty_config_derive::ConfigDeserialize;
use serde::Serialize;

#[derive(ConfigDeserialize, Serialize, Copy, Clone, Debug, PartialEq)]
pub struct SelTools {
    pub enabled:    bool,
    pub hint_width: usize,

    pub parse_json: bool,
}

impl Default for SelTools {
    fn default() -> Self {
        Self {
            enabled:    false,
            hint_width: 40,
            parse_json: true,
        }
    }
}
