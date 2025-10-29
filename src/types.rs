#[derive(Debug)]
pub enum MusicType {
    DefaultMusicExtension,
    WavExtension,
    FlacExtension,
    MPThreeExtension,
    None,
}

#[derive(Debug)]
pub enum CoverArtType {
    PngExtension,
    JpegExtension,
    JpgExtension,
    None,
}
