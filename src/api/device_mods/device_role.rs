use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum DeviceRole {
    #[serde(rename = "instrument")]
    Instrument,
    #[serde(rename = "noteFX")]
    NoteFX,
    #[serde(rename = "audioFX")]
    AudioFX,
    #[serde(rename = "analyzer")]
    Analyzer,
}
