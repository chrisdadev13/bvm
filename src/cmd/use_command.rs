use crate::versions::Versions;

pub struct Use;
impl Use {
    pub fn use_command(version: String) {
        Versions::switch(version.clone());
        println!("Now using Bun {}", version);
    }
}
