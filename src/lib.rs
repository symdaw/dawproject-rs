use std::io::{Read, Write};

use crate::api::{meta_data::MetaData, project::Project};

pub mod api;

const PROJECT_PATH: &str = "project.xml";
const METADATA_PATH: &str = "metadata.xml";

pub fn load(
    path: impl AsRef<std::path::Path>,
) -> Result<(Option<Project>, Option<MetaData>), Box<dyn std::error::Error>> {
    let file = std::fs::File::open(path)?;
    let mut archive = zip::ZipArchive::new(file)?;

    let mut proj = None;
    let mut meta = None;

    if let Ok(mut proj_str) = archive.by_name(PROJECT_PATH) {
        let mut buf = String::new();
        proj_str.read_to_string(&mut buf)?;
        proj = Some(quick_xml::de::from_str::<Project>(&buf)?);
    }
    if let Ok(mut meta_str) = archive.by_name(METADATA_PATH) {
        let mut buf = String::new();
        meta_str.read_to_string(&mut buf)?;
        meta = Some(quick_xml::de::from_str::<MetaData>(&buf)?);
    }

    Ok((proj, meta))
}

pub fn save(
    project: Option<Project>,
    metadata: Option<MetaData>,
    path: impl AsRef<std::path::Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::create(path)?;
    let mut zip = zip::ZipWriter::new(file);
    if let Some(project) = project {
        zip.start_file(
            PROJECT_PATH,
            zip::write::FileOptions::default()
                .compression_method(zip::CompressionMethod::Stored)
                .unix_permissions(0o755),
        )?;
        zip.write_all(quick_xml::se::to_string(&project)?.as_bytes())?;
    }
    if let Some(metadata) = metadata {
        zip.start_file(
            METADATA_PATH,
            zip::write::FileOptions::default()
                .compression_method(zip::CompressionMethod::Stored)
                .unix_permissions(0o755),
        )?;
        zip.write_all(quick_xml::se::to_string(&metadata)?.as_bytes())?;
    }
    zip.finish()?;

    Ok(())
}
