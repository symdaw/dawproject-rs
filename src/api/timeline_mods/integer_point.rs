use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct IntegerPoint {
    #[serde(rename = "@time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(rename = "$value", default)]
    pub integer_point_sequence: Option<Vec<i32>>,
}
