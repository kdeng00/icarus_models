use std::io::Write;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize, utoipa::ToSchema)]
pub struct CoverArt {
    pub id: uuid::Uuid,
    pub title: String,
    #[serde(skip)]
    pub directory: String,
    pub filename: String,
    #[serde(skip)]
    pub data: Vec<u8>,
    pub song_id: uuid::Uuid,
}

pub mod init {
    use super::CoverArt;

    /// Initializes the CoverArt with just the directory and filename
    pub fn init_coverart_dir_and_filename(directory: &str, filename: &str) -> CoverArt {
        CoverArt {
            directory: String::from(directory),
            filename: String::from(filename),
            ..Default::default()
        }
    }
}

impl CoverArt {
    /// Saves the coverart to the filesystem
    pub fn save_to_filesystem(&self) -> Result<(), std::io::Error> {
        match self.get_path() {
            Ok(path) => {
                match std::fs::File::create(&path) {
                    Ok(mut file) => match file.write_all(&self.data) {
                        Ok(_) => Ok(()),
                        Err(err) => Err(err),
                    },
                    Err(err) => Err(err),
                }
            }
            Err(err) => Err(err)
        }
    }

    /// Removes the coverart from the filesystem
    pub fn remove_from_filesystem(&self) -> Result<(), std::io::Error> {
        match self.get_path() {
            Ok(path) => {
                let p = std::path::Path::new(&path);
                if p.exists() {
                    match std::fs::remove_file(p) {
                        Ok(_) => Ok(()),
                        Err(err) => Err(err),
                    }
                } else {
                    Err(std::io::Error::other(
                        "Cannot delete file that does not exist",
                    ))
                }
            }
            Err(err) => Err(err)
        }
    }

    /// Gets the path of the CoverArt
    pub fn get_path(&self) -> Result<String, std::io::Error> {
        if self.directory.is_empty() {
            return Err(std::io::Error::other("Directory has not been initialized"));
        } else if self.filename.is_empty() {
            return Err(std::io::Error::other("Filename has not bee initialized"));
        }

        let directory = &self.directory;
        let last_index = directory.len() - 1;

        if let Some(character) = directory.chars().nth(last_index) {
            let buffer = if character != '/' {
                directory.clone() + "/"
            } else {
                directory.clone()
            };

            Ok(buffer + &self.filename.clone())
        } else {
            Err(std::io::Error::other(
                "Could not access last character of directory",
            ))
        }
    }
}

pub mod io {
    use std::io::Read;

    /// Gets the raw data of the cover art
    pub fn to_data(coverart: &super::CoverArt) -> Result<Vec<u8>, std::io::Error> {
        match coverart.get_path() {
            Ok(path) => {
                let mut file = std::fs::File::open(path)?;
                let mut buffer = Vec::new();
                match file.read_to_end(&mut buffer) {
                    Ok(_) => Ok(buffer),
                    Err(err) => Err(err),
                }
            }
            Err(err) => Err(err)
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
