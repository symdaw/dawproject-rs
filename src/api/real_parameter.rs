use serde::{Deserialize, Serialize};

use super::unit::Unit;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RealParameter {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(rename = "@unit")]
    pub unit: Unit,
    #[serde(rename = "@min")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(rename = "@max")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
}
