use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BoolPoint {
    #[serde(rename = "@time")]
    pub time: Vec<String>,

    #[serde(rename = "@value")]
    pub value: bool,
}
