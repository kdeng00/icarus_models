mod utils {
    use std::fs;
    use std::io::Read;
    use std::path::Path;

    pub fn get_tests_directory() -> String {
        String::from(env!("CARGO_MANIFEST_DIR").to_owned() + "/tests/")
    }

    pub fn does_directory_exists(directory: &String) -> bool {
        let path = Path::new(directory);
        if let Ok(dir_i) = fs::metadata(path) {
            dir_i.is_dir()
        } else {
            false
        }
    }

    pub fn extract_data_from_file(filepath: &String) -> Result<Vec<u8>, std::io::Error> {
        match std::fs::File::open(filepath) {
            Ok(mut file) => {
                let mut buffer: Vec<u8> = Vec::new();
                let _ = file.read_to_end(&mut buffer);
                Ok(buffer)
            }
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod song_tests {
    use std::fs::File;
    use std::io::Write;

    use tempfile::tempdir;

    use icarus_models::song;
    use icarus_models::types;

    use crate::utils;

    #[test]
    fn test_song_to_data() {
        println!("Test");
        let some_val = true;

        println!("Checking if some_val is true");
        assert_eq!(true, some_val);

        println!("Getting track");
        let mut song = song::Song::default();
        song.directory = utils::get_tests_directory();
        song.filename = String::from("track01.flac");

        assert!(
            utils::does_directory_exists(&song.directory),
            "Directory does not exist"
        );

        println!("Directory: {}", song.directory);

        match song.song_path() {
            Ok(filepath) => match utils::extract_data_from_file(&filepath) {
                Ok(buffer) => {
                    assert_eq!(buffer.is_empty(), false);

                    match song.to_data() {
                        Ok(song_data) => {
                            println!("Both files match");
                            assert_eq!(buffer, song_data);
                        }
                        Err(err) => {
                            assert!(false, "Error producing song data: {:?}", err);
                        }
                    };
                }
                Err(err) => {
                    assert!(false, "Failed to open file: {:?}", err);
                }
            },
            Err(err) => {
                assert!(false, "Could not get song path: {:?}", err);
            }
        }
    }

    #[test]
    fn test_song_path_check() {
        let mut song = song::Song::default();
        song.directory = utils::get_tests_directory();
        song.filename = String::from("track01.flac");

        assert!(
            utils::does_directory_exists(&song.directory),
            "Directory does not exist"
        );
    }

    #[test]
    fn test_song_generate_filename() {
        let mut song = song::Song::default();
        song.directory = utils::get_tests_directory();
        song.filename = String::from("track01.flac");

        match song.song_path() {
            Ok(songpath) => match utils::extract_data_from_file(&songpath) {
                Ok(buffer) => {
                    let mut song_cpy = song.clone();
                    let temp_dir = tempdir().expect("Failed to create temp dir");
                    song_cpy.directory = match temp_dir.path().to_str() {
                        Some(s) => String::from(s),
                        None => String::new(),
                    };

                    assert_eq!(song.directory.is_empty(), false);
                    song_cpy.filename =
                        song.generate_filename(types::MusicTypes::FlacExtension, true);
                    println!("Directory: {:?}", song_cpy.directory);
                    println!("File to be created: {:?}", song_cpy.filename);

                    let path = match song_cpy.song_path() {
                        Ok(s_path) => s_path,
                        Err(err) => {
                            assert!(false, "Error: {:?}", err);
                            String::new()
                        }
                    };

                    match File::create(path) {
                        Ok(mut file_cpy) => match file_cpy.write_all(&buffer) {
                            Ok(success) => {
                                println!("Success: {:?}", success);
                            }
                            Err(err) => {
                                assert!(false, "Error saving file: {:?}", err);
                            }
                        },
                        Err(err) => {
                            assert!(false, "Error: {:?}", err);
                        }
                    };
                }
                Err(err) => {
                    assert!(false, "Error: {:?}", err);
                }
            },
            Err(err) => {
                assert!(false, "Error extracting song data: {:?}", err);
            }
        }
    }

    #[test]
    fn test_save_song_to_filesystem() {
        let mut song = song::Song::default();
        song.directory = utils::get_tests_directory();
        song.filename = String::from("track02.flac");

        match song.song_path() {
            Ok(song_path) => {
                match utils::extract_data_from_file(&song_path) {
                    Ok(data) => {
                        let copied_song = song::Song {
                            directory: utils::get_tests_directory(),
                            filename: String::from("track02-coppied.flac"),
                            data: data,
                            ..Default::default()
                        };

                        match copied_song.save_to_filesystem() {
                            Ok(_) => {
                            }
                            Err(err) => {
                                assert!(false, "Error: {err:?}")
                            }
                        }
                    }
                    Err(err) => {
                        assert!(false, "Error: {err:?}")
                    }
                }
            }
            Err(err) => {
                assert!(false, "Error: {err:?}");
            }
        }
    }
}

#[cfg(test)]
mod album_tests {

    use crate::utils;
    use icarus_models::album;

    #[test]
    fn parse_album() {
        let test_dir = utils::get_tests_directory();
        if utils::does_directory_exists(&test_dir) {
            let album_file: String = test_dir + &String::from("album.json");
            println!("Album file: {:?}", album_file);

            match album::collection::parse_album(&album_file) {
                Ok(album) => {
                    println!("Album title: {}", album.title);
                    assert_eq!(album.title.is_empty(), false);
                    assert_eq!(album.artist.is_empty(), false);
                    assert_eq!(album.tracks.is_empty(), false);
                }
                Err(err) => {
                    assert!(false, "Error parsing album json file: {:?}", err);
                }
            }
        }
    }
}
