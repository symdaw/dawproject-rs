use {
    super::{device::DeviceElements, device_role::DeviceRole},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ClapPlugin {
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(rename = "@pluginVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_version: Option<String>,
}
