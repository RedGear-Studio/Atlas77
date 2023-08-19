#[derive(Debug, Clone, PartialEq)]
pub struct FilePath {
    pub path: String,
}

impl FilePath {
    //Return the content of the file as a string slice
    pub fn get_code(self) -> Result<String, std::io::Error> {
        let file_content = std::fs::read_to_string(self.path)?;
        Ok(file_content)
    }

    pub fn get_file_name(path: &str) -> &str {
        path.split('/').last().unwrap()
    }
}