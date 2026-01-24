use crate::config::Config;

#[derive(Clone, Debug)]
pub struct Game {
    pub name: String,
    pub release: String,
    pub image: String,
    pub platforms: Vec<String>,
    pub description: Option<String>,
}

impl Game {
    pub fn random(config: &Config) -> Game {
        todo!()
    }
}
