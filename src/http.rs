use serde::{Deserialize, Serialize};
use ureq::{get, Error};

#[derive(Serialize, Deserialize, Debug)]
pub struct BunTag {
    #[serde(rename = "ref")]
    pub ref_field: String,
}

pub struct HTTPRequest;
impl HTTPRequest {
    pub fn bun_registry() -> Result<Vec<BunTag>, Error> {
        let body: String = get("https://api.github.com/repos/oven-sh/bun/git/refs/tags")
            .call()?
            .into_string()?;

        let json: Vec<BunTag> = serde_json::from_str(body.as_str()).unwrap();

        Ok(json)
    }
}
