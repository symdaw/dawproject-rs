
use {
    
        serde::{Deserialize, Serialize},
};
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Marker {
    #[serde(rename = "@name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(rename = "@color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
    #[serde(rename = "@comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,

    #[serde(rename = "@time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    time: Option<Vec<f64>>,
}


