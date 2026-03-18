use {
    serde::{Deserialize, Serialize},
};
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BoolParameter {
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
    #[serde(rename = "@parameterID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_id: Option<i32>,
    #[serde(rename = "@value")]
    pub value: Option<bool>,
}
