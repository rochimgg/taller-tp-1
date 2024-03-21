use std::fs::File;
use std::io::{self, Read};
pub struct Reader {
    file_path: String,
}

impl Reader {
    pub fn new(file_path: String) -> Self {
        Reader { file_path }
    }

    pub fn read_file(&self) -> Result<String, io::Error> {
        let mut file = File::open(&self.file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }
}
