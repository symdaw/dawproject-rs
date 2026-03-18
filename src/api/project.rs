use super::{application::Application, arrangement::Arrangement, scene::Scene};

use super::transport::Transport;

use super::{channel::Channel, track::Track};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum TrackChannelEnum {
    Track(Track),
    Channel(Channel),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Structure {
    #[serde(rename = "$value", default)]
    pub sequence: Vec<TrackChannelEnum>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Project {
    #[serde(rename = "@version")]
    pub version: String,
    #[serde(rename = "Application")]
    pub application: Application,
    #[serde(rename = "Transport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport: Option<Transport>,
    #[serde(rename = "Structure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure: Option<Structure>,
    #[serde(rename = "Arrangement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrangement: Option<Arrangement>,
    #[serde(rename = "Scenes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenes: Option<Vec<Scene>>,
}
