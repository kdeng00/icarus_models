use std::io::Read;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CoverArt {
    pub id: i32,
    pub title: String,
    pub path: String,
    pub data: Vec<u8>,
}

impl CoverArt {
    pub fn to_data(&self) -> Result<Vec<u8>, std::io::Error> {
        let path: &String = &self.path;
        let mut file = std::fs::File::open(path)?;
        let mut buffer = Vec::new();
        match file.read_to_end(&mut buffer) {
            Ok(_) => Ok(buffer),
            Err(err) => Err(err),
        }
    }
}
