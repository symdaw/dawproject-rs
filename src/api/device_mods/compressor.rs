use super::{device::DeviceElements, device_role::DeviceRole};

use {
    super::{super::bool_parameter::BoolParameter, super::real_parameter::RealParameter},
    serde::{Deserialize, Serialize},
};
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum CompressorParamsEnum {
    Attack(RealParameter),
    AutoMakeup(BoolParameter),
    InputGain(RealParameter),
    OutputGain(RealParameter),
    Ratio(RealParameter),
    Release(RealParameter),
    Threshold(RealParameter),
}

pub type CompressorParams = Vec<CompressorParamsEnum>;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Compressor {
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "$value", default)]
    pub device_elements: DeviceElements,
    #[serde(rename = "@deviceID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "@deviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(rename = "@deviceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_role: Option<DeviceRole>,
    #[serde(rename = "@deviceVendor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_vendor: Option<String>,
    #[serde(rename = "@loaded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loaded: Option<bool>,
    #[serde(rename = "$value", default)]
    pub compressor_elements: Vec<CompressorParamsEnum>,
}
