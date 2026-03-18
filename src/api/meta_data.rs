use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum MetaDataEnum {
    Title(String),
    Artist(String),
    Album(String),
    OriginalArtist(String),
    Composer(String),
    Songwriter(String),
    Producer(String),
    Arranger(String),
    Year(String),
    Genre(String),
    Copyright(String),
    Website(String),
    Comment(String),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MetaData {
    #[serde(rename = "$value", default)]
    pub meta_data: Vec<MetaDataEnum>,
}
