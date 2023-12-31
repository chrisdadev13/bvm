use crate::http::HTTPRequest;

pub struct LsRemoteCommand;
impl LsRemoteCommand {
    pub fn list_remote_versions() {
        match HTTPRequest::bun_registry() {
            Ok(res) => {
                for tag in &res[11..res.len() - 8] {
                    let version = tag
                        .ref_field
                        .rsplit('/')
                        .next()
                        .unwrap_or("error getting version")
                        .strip_prefix("bun-")
                        .unwrap();
                    println!("   {}", version);
                }
            }
            Err(_) => {
                eprint!("error fetching remote versions")
            }
        }
    }
}
