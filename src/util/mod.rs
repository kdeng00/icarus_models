pub fn concatenate_path(
    directory: &str,
    filename: &str,
    last_index: usize,
) -> Result<String, std::io::Error> {
    if let Some(character) = directory.chars().nth(last_index) {
        let buffer: String = if character != '/' {
            format!("{directory}/")
        } else {
            String::from(directory)
        };

        Ok(format!("{buffer}{filename}"))
    } else {
        Err(std::io::Error::other(
            crate::constants::error::LAST_CHARACTER_IN_DIRECTORY,
        ))
    }
}
