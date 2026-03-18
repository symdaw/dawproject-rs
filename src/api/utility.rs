
use {
    super::{
        content_type::ContentType,
        mixer_role::MixerRoleEnum,
        timeline_mods::{audio::Audio, clip::Clip, clips::Clips, timeline::TimeLine, warp::Warp},
        track::Track,
    },
    std::{error::Error, fs::File, path::Path},
};

pub fn create_file_path_absolute_string(file_path: String) -> Result<String, Box<dyn Error>> {
    match File::create(file_path.clone()) {
        Ok(f) => (),
        Err(err) => return Err(err.into()),
    };

    match std::fs::canonicalize(Path::new(&file_path)) {
        Ok(p) => return Ok(p.to_str().unwrap().to_string()),
        Err(err) => Err(err.into()),
    }
}
