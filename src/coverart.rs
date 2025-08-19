use std::io::Read;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize, utoipa::ToSchema)]
pub struct CoverArt {
    pub id: uuid::Uuid,
    pub title: String,
    #[serde(skip)]
    pub path: String,
    #[serde(skip)]
    pub data: Vec<u8>,
    pub song_id: uuid::Uuid,
}

pub mod init {
    use crate::coverart::CoverArt;

    pub fn init_coverart_only_path(path: String) -> CoverArt {
        CoverArt {
            id: uuid::Uuid::nil(),
            title: String::new(),
            path: path.clone(),
            data: Vec::new(),
            song_id: uuid::Uuid::nil(),
        }
    }
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

#[cfg(test)]
mod tests {
    use crate::coverart;

    #[test]
    fn test_cover_art_image() {
        let path: String = String::from("somepath");
        let coverart = coverart::init::init_coverart_only_path(path.clone());

        assert_eq!(path, coverart.path);
    }
}
