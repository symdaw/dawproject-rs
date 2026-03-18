use {
    super::eq_band_type::EqBandTypeEnum,
    super::{super::bool_parameter::BoolParameter, super::real_parameter::RealParameter},
    serde::{Deserialize, Serialize},
};
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum EqBandParamsEnum {
    Freq(RealParameter),
    Gain(RealParameter),
    Q(RealParameter),
    Enabled(BoolParameter),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EqBand {
    #[serde(rename = "@type")]
    pub eq_type: EqBandTypeEnum,
    #[serde(rename = "@order")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[serde(rename = "$value", default)]
    pub eq_band_params: Vec<EqBandParamsEnum>,
}
