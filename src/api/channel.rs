#![allow(clippy::large_enum_variant)]

use {
    super::{
        super::api::device_mods::{
            au_plugin::AuPlugin, builtin_device::BuiltinDevice, clap_plugin::ClapPlugin,
            compressor::Compressor, device::Device, equalizer::Equalizer, limiter::Limiter,
            noise_gate::NoiseGate, vst2_plugin::Vst2Plugin, vst3_plugin::Vst3Plugin,
        },
        bool_parameter::BoolParameter,
        mixer_role::MixerRoleEnum,
        real_parameter::RealParameter,
        send::Send,
    },
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum DeviceTypes {
    Device(Device),
    Vst2Plugin(Vst2Plugin),
    Vst3Plugin(Vst3Plugin),
    ClapPlugin(ClapPlugin),
    BuiltinDevice(BuiltinDevice),
    Equalizer(Equalizer),
    Compressor(Compressor),
    NoiseGate(NoiseGate),
    Limiter(Limiter),
    AuPlugin(AuPlugin),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ChannelParameters {
    Volume(Option<RealParameter>),
    Pan(Option<RealParameter>),
    Mute(Option<BoolParameter>),
    Devices(Option<Devices>),
    Sends(Option<Vec<Send>>),
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Devices {
    #[serde(rename = "$value")]
    pub choice: Vec<DeviceTypes>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Channel {
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "@name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "@color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "@comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "@role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<MixerRoleEnum>,
    #[serde(rename = "@audioChannels")]
    pub audio_channels: i32,
    #[serde(rename = "@solo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solo: Option<bool>,
    #[serde(rename = "@destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(rename = "$value", default)]
    pub elements: Option<Vec<ChannelParameters>>,
}
