
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename = "camelCase")]
pub enum ExpressionTypeEnum {
    Gain,
    Pan,
    Transpose,
    Timbre,
    Formant,
    Pressure,
    ChannelController,
    ChannelPressure,
    PolyPressure,
    PitchBend,
    ProgramChange,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExpressionType {
    #[serde(rename = "$value", default)]
    pub expression_type: Vec<ExpressionTypeEnum>,
}
