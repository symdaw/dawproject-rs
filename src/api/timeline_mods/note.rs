use {
    super::lanes::ArrangementTypeChoiceEnum,
    serde::{Deserialize, Serialize},
};
type NoteSequenceChoice = Vec<ArrangementTypeChoiceEnum>;
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Note {
    #[serde(rename = "@time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<f64>,
    #[serde(rename = "@duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    #[serde(rename = "@channel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<i32>,
    #[serde(rename = "@key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<i32>,
    #[serde(rename = "@velocity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vel: Option<f64>,
    #[serde(rename = "@releaseVelocity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rel: Option<f64>,
    #[serde(rename = "$value", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes_sequence_choice: Option<NoteSequenceChoice>,
}
