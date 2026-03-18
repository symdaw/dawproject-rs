use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum EqBandTypeEnum {
    #[serde(rename = "highPass")]
    HighPass,
    #[serde(rename = "lowPass")]
    LowPass,
    #[serde(rename = "bandPass")]
    BandPass,
    #[serde(rename = "highShelf")]
    HighShelf,
    #[serde(rename = "lowShelf")]
    LowShelf,
    #[serde(rename = "bell")]
    Bell,
    #[serde(rename = "@notch")]
    Notch,
}
