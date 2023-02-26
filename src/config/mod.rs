use std::path::PathBuf;
use crate::osm::pbf::file_info::FileInfo;

pub struct Config {
    pub input: PathBuf,
    pub input_format: String,
    pub output: PathBuf,
    pub output_format: String,
    pub file_info: FileInfo,
}

impl Config {
    pub fn new(input: PathBuf,
               input_format: String,
               output: PathBuf,
               output_format: String,
               file_info: FileInfo,
    ) -> Config {
        Config {
            input,
            input_format,
            output,
            output_format,
            file_info,
        }
    }
}