use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "lowercase")]
pub enum SendTypeEnum {
    #[default]
    Pre,
    Post,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct SendType {
    #[serde(rename = "$value", default)]
    field: Vec<SendTypeEnum>,
}
