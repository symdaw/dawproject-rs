use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum WarpSequenceEnum {
    Time(f64),
    ContentTime(f64),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Warp {
    #[serde(rename = "@time")]
    pub time: f64,
    #[serde(rename = "@contentTime")]
    pub content_time: f64,
}
