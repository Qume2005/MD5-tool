use std::{fs::File, io::Read, path::PathBuf};

use anyhow::{Context, Result};

#[derive(Clone)]
pub enum Target {
    File(PathBuf),
    Text(String),
}

impl Default for Target {
    fn default() -> Self {
        Target::Text("MD5 tool".to_string())
    }
}

#[derive(Default)]
pub struct State {
    edit_mode_status: bool,
    target: Target,
    result: String,
    prev_result: String,
}

impl State {
    pub fn get_data(&self) -> Result<Vec<u8>> {
        match &self.target {
            Target::File(path) => {
                let mut file = File::open(path).with_context(|| format!("Failed to open file: {:?}", path))?;
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer).with_context(|| "Failed to read file content")?;
                Ok(buffer)
            }
            Target::Text(text) => {
                Ok(text.as_bytes().to_vec())
            }
        }
    }

    pub fn enable_edit_mode(&mut self) {
        self.edit_mode_status = true;
    }

    pub fn disable_edit_mode(&mut self) {
        self.edit_mode_status = false;
    }

    pub fn set_target(&mut self, target: Target) {
        self.target = target;
    }

    pub fn get_target(&self) -> String {
        match &self.target {
            Target::File(path) => path.to_str().unwrap_or("").to_string(),
            Target::Text(text) => text.to_owned(),
        }
    }

    pub fn set_result(&mut self, result: String) {
        (self.prev_result, self.result) = (self.result.clone(), result);
    }

    pub fn get_result(&self) -> String {
        self.result.clone()
    }

    pub fn get_prev_result(&self) -> String {
        self.prev_result.clone()
    }

    pub fn get_edit_mode_status(&self) -> bool {
        self.edit_mode_status
    }
}