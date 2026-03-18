use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum MixerRoleEnum {
    #[serde(rename = "regular")]
    Regular,
    #[serde(rename = "master")]
    Master,
    #[serde(rename = "effect")]
    Effect,
    #[serde(rename = "submix")]
    SubMix,
    #[serde(rename = "vca")]
    Vca,
}
