
use {
    super::{real_parameter::RealParameter, send_type::SendTypeEnum},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Send {
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Volume")]
    pub volume: RealParameter,
    #[serde(rename = "Pan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pan: Option<RealParameter>,
    #[serde(rename = "@destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(rename = "@type")]
    pub send_type: SendTypeEnum,
}
