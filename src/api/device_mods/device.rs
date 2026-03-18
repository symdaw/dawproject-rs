use super::device_role::DeviceRole;

use {
    super::{super::bool_parameter::BoolParameter, super::file_reference::FileReference},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum DeviceElementsEnum {
    Parameters,
    Enabled(BoolParameter),
    State(FileReference),
}

pub type DeviceElements = Vec<DeviceElementsEnum>;

#[derive(Debug, Deserialize, Serialize, Clone)]
enum Parameters {
    #[serde(rename = "parameter")]
    Parameter,
    #[serde(rename = "realParameter")]
    RealParameter,
    #[serde(rename = "boolParameter")]
    BoolParameter,
    #[serde(rename = "integerParameter")]
    IntegerParameter,
    #[serde(rename = "enumParameter")]
    EnumParameter,
    #[serde(rename = "timeSignatureParameter")]
    TimeSignatureParameter,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Device {
    // Extends referenceable
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
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
    pub device_elements: DeviceElements,
}
