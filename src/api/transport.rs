use super::{real_parameter::RealParameter, time_signature_parameter::TimeSignatureParameter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum TransportSequence {
    Tempo(RealParameter),
    TimeSignature(TimeSignatureParameter),
}

type TransportSequenceVec = Vec<TransportSequence>;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Transport {
    #[serde(rename = "$value", default)]
    pub sequence: TransportSequenceVec,
}
