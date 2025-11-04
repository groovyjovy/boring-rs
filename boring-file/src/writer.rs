use serde::Serialize;
use std::{fs, path::PathBuf};

pub struct FileWriter {}

impl FileWriter {
    pub fn write_json<T: Serialize>(
        data: &T,
        output_file: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(parent_dir) = output_file.parent() {
            if !parent_dir.exists() {
                fs::create_dir_all(parent_dir)?;
            }
        }

        let json = serde_json::to_string_pretty(data)?;
        fs::write(&output_file, json)?;

        println!("JSONファイルを出力しました: {:?}", &output_file);

        Ok(())
    }
}
