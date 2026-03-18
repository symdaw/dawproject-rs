use std::{error::Error, fs::File, path::Path};

pub fn create_file_path_absolute_string(file_path: String) -> Result<String, Box<dyn Error>> {
    match File::create(file_path.clone()) {
        Ok(_f) => (),
        Err(err) => return Err(err.into()),
    };

    match std::fs::canonicalize(Path::new(&file_path)) {
        Ok(p) => Ok(p.to_str().unwrap().to_string()),
        Err(err) => Err(err.into()),
    }
}
