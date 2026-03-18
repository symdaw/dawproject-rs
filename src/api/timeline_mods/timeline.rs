use {
    super::super::timeline_mods::time_unit::TimeUnit,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TimeLine {
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "@name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "@color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "@comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "@track")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track: Option<String>,
    #[serde(rename = "@timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub time_unit: Option<TimeUnit>,
}
