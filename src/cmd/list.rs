use crate::http::HTTPRequest;

pub struct LsCommand;
impl LsCommand {
    pub fn list_remote_versions() {
        match HTTPRequest::bun_registry() {
            Ok(res) => {
                for tag in &res[11..res.len() - 8] {
                    let version = tag.ref_field.to_string()[14..].to_string();
                    println!("   {}", version);
                }
            }
            Err(_) => {
                eprint!("error fetching remote versions")
            }
        }
    }
}
