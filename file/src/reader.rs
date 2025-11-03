use clap::{Parser, crate_authors};
use encoding_rs::SHIFT_JIS;
use std::{fs::File, io::Read, path::PathBuf, str};

#[derive(Parser, Debug)]
#[command(
    author = crate_authors!(", \n"),
    version,
    long_about = None,
    help_template = "{author-with-newline}{about-section}Version: {version}\n{usage-heading} {usage}\n{all-args}{tab}"
)]
pub struct FileReader {
    #[arg(
        short,
        long,
        value_name = "INPUT_FILE",
        help = "読み込むXMLファイルのパス"
    )]
    pub input_file: PathBuf,
    #[arg(long, value_name = "OUTPUT_FILE", help = "出力ファイルパス")]
    pub output_file: PathBuf,
}

impl FileReader {
    pub fn from_args() -> Self {
        Self::parse()
    }

    pub fn read_xml(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut file = File::open(&self.input_file)?;
        let mut raw_bytes = Vec::new();
        file.read_to_end(&mut raw_bytes)?;
        Ok(raw_bytes)
    }

    pub fn shift_jis_to_utf8(raw_bytes: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let (cow, _, _) = SHIFT_JIS.decode(raw_bytes);
        Ok(cow.as_bytes().to_vec())
    }
}
