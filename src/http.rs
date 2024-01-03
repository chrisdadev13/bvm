use std::fs::File;
use std::path::Path;
use std::{error::Error, io::Write};

use serde::{Deserialize, Serialize};
use ureq::get;

#[derive(Serialize, Deserialize, Debug)]
pub struct BunTag {
    #[serde(rename = "ref")]
    pub ref_field: String,
}

pub struct HTTPRequest;
impl HTTPRequest {
    pub fn download_zip(url: &str, path: &Path) -> Result<Vec<u8>, Box<dyn Error>> {
        let res = get(url).call();

        if res.is_err() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to download file",
            )));
        }

        let mut dir = File::create(path)?;
        let mut buffer: Vec<u8> = Vec::new();
        res?.into_reader().read_to_end(&mut buffer)?;

        dir.write_all(&buffer)?;

        println!("Version downloaded successfully");

        Ok(buffer)
    }
    pub fn bun_registry() -> Result<Vec<BunTag>, Box<dyn Error>> {
        let res = get("https://api.github.com/repos/oven-sh/bun/git/refs/tags")
            .call()?
            .into_string();

        let json: Vec<BunTag> = serde_json::from_str(res?.as_str())?;

        Ok(json)
    }
}
