use crate::vlink::VLink;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub fn load_settings(file: &str) -> Option<AppSettings> {
    use std::fs::read;
    use std::io::{ErrorKind::NotFound, Error};
    use std::io::Result;
    read(file)
        .or( dirs::home_dir().and_then(|home| { Some(read(home.join(".v2ray-maid.json"))) }).unwrap_or(Result::Err(Error::new(NotFound, "not found"))))
        .or( dirs::config_dir().and_then(|conf| { Some(read(conf.join("v2ray-maid.json"))) }).unwrap_or(Result::Err(Error::new(NotFound, "not found"))))
        .or(read(Path::new("/etc/v2ray-maid").join("v2ray-maid.json")))
        .map(|buf| Some(serde_json::from_slice::<AppSettings>(buf.as_ref()).unwrap()))
        .unwrap_or(None)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppSettings {
    pub sub_url: String,
    #[serde(default = "default_loglevel")]
    pub loglevel: String,
    #[serde(default = "default_program")]
    pub program: String,
    pub ping_times: Option<i32>,
    pub proxies: Vec<VlinkProxy>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VlinkProxy {
    pub selector: String,
    pub tag: Option<String>,
    pub target_file: String,
    pub limit: Option<usize>,
    #[serde(skip_serializing, default = "Vec::new")]
    pub vlinks: Vec<VLink>,
}

impl VlinkProxy {
    pub fn add_link(&mut self, vlink: VLink) {
        self.vlinks.push(vlink);
    }
}

fn default_loglevel() -> String {
    "info".to_string()
}

#[cfg(target_family = "windows")]
fn default_program1() -> String {
    "v2ray.exe".to_string()
}

#[cfg(target_family = "unix")]
fn default_program() -> String {
    "v2ray".to_string()
}
