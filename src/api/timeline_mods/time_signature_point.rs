use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum TimeSignaturePointEnum {
    #[serde(rename = "@numerator")]
    Numerator(i32),
    #[serde(rename = "@denominator")]
    Denominator(i32),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TimeSignaturePoint {
    #[serde(rename = "@time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,

    #[serde(rename = "$value", default)]
    pub real_point_sequence: Option<Vec<TimeSignaturePointEnum>>,
}
