use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum InterpolationEnum {
    Hold,
    Linear,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Interpolation {
    #[serde(rename = "$value", default)]
    pub interpolation_type: Vec<InterpolationEnum>,
}
