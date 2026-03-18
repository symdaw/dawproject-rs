use serde::{Deserialize, Serialize};

use crate::api::InterpolationEnum;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RealPoint {
    #[serde(rename = "@time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(rename = "@value")]
    pub value: Option<String>,
    #[serde(rename = "@interpolation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpolation: Option<InterpolationEnum>,
}
