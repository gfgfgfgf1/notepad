use anyhow::Result;
use std::path::{Path, PathBuf};

pub struct OpenFile {
    pub path: Option<PathBuf>,
    pub name: String,
    pub changed: bool,
    pub content_buffer: String,
}

impl Default for OpenFile {
    fn default() -> Self {
        OpenFile {
            path: None,
            name: "Unnamed".to_string(),
            changed: false,
            content_buffer: "".to_string(),
        }
    }
}

impl OpenFile {
    pub fn open_path(path: &Path) -> Result<OpenFile> {
        let file_name = path
            .file_name()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "File not found"))?
            .to_string_lossy();
        let content_buffer = std::fs::read_to_string(path)?;
        Ok(OpenFile {
            name: file_name.to_string(),
            path: Some(path.to_owned()),
            changed: false,
            content_buffer,
        })
    }

    pub fn save(&mut self) -> Result<()> {
        let unwrapped_path = self.path.as_deref().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, "File path doesn't exist")
        })?;
        std::fs::write(unwrapped_path, &self.content_buffer)?;
        self.changed = false;
        Ok(())
    }
}
