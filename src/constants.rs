pub mod file_extensions {
    pub mod audio {
        pub const DEFAULTMUSICEXTENSION: &str = FLACEXTENSION;
        pub const FLACEXTENSION: &str = ".flac";
        pub const WAVEXTENSION: &str = ".wav";
        pub const MPTHREEEXTENSION: &str = ".mp3";
    }

    pub mod image {
        pub const JPGEXTENSION: &str = ".jpg";
        pub const JPEGEXTENSION: &str = ".jpeg";
        pub const PNGEXTENSION: &str = ".png";
    }
}

pub mod error {
    pub const DIRECTORY_NOT_INITIALIZED: &str = "Directory has not been initialized";
    pub const FILENAME_NOT_INITIALIZED: &str = "Filename has not bee initialized";
    pub const LAST_CHARACTER_IN_DIRECTORY: &str = "Could not access last character of directory";
}
