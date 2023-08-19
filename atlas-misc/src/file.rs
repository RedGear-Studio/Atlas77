#[derive(Debug, Clone, Copy)]
pub struct FilePath<'a> {
    pub path: &'a str,
    pub file_name: &'a str,
}

impl<'a> FilePath<'a> {
    //Return the content of the file as a string slice
    pub fn get_code(self) -> Result<String, std::io::Error> {
        let file_content = std::fs::read_to_string(self.path)?;
        Ok(file_content)
    }
}