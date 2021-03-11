use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Deserialize, Serialize)]
pub struct VLink {
    pub protocol: String,
    pub address: String,
    pub port: u16,
    pub id: String,
    pub alter_id: i32,
    pub security: String,
    pub network: String,
    pub remarks: String,
    pub header_type: String,
    pub request_host: String,
    pub path: String,
    #[serde(alias = "streamSecurity")]
    pub stream_security: String,
    #[serde(skip_serializing)]
    pub latency: i32,
}

impl Default for VLink {
    fn default() -> Self {
        Self {
            protocol: "vmess".to_string(),
            address: "v2ray.cool".to_string(),
            port: 10086,
            id: "bf0067d4-831e-4911-b644-9b4582f69671".to_string(),
            alter_id: 64,
            security: "".to_string(),
            network: "tcp".to_string(),
            remarks: "def".to_string(),
            header_type: "".to_string(),
            request_host: "".to_string(),
            path: "".to_string(),
            stream_security: "".to_string(),
            latency: -1,
        }
    }
}

impl fmt::Debug for VLink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.remarks)
    }
}
