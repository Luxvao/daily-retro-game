use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Config {
    // SMTP stuff
    pub src_email: String,
    pub src_email_auth: String,
    pub smtp_server: String,
    pub dest_email: String,

    // RAWG
    pub rawg_key: String,
    pub platforms: Vec<u8>,
    pub genres: Vec<u8>,

    // Timing
    pub cron: String,
}
