use {
    super::super::expression_type::ExpressionTypeEnum,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AutomationTarget {
    #[serde(rename = "@parameter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter: Option<String>,
    #[serde(rename = "@expression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<ExpressionTypeEnum>,
    #[serde(rename = "@channel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<i32>,
    #[serde(rename = "@key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<i32>,
    #[serde(rename = "@controller")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller: Option<i32>,
}
