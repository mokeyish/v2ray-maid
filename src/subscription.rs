use crate::vlink::VLink;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct VmessShare {
    pub host: String,
    pub path: String,
    pub tls: String,
    pub verify_cert: bool,
    pub add: String,
    pub port: u16,
    pub aid: i32,
    pub net: String,
    pub r#type: String,
    pub v: String,
    pub ps: String,
    pub id: String,
    pub class: i32,
}

impl Into<VLink> for VmessShare {
    fn into(self) -> VLink {
        let mut vlink = VLink::default();
        vlink.security = "auto".to_string();
        vlink.network = "tcp".to_string();
        vlink.header_type = "none".to_string();

        vlink.remarks = self.ps;
        vlink.address = self.add;
        vlink.port = self.port;
        vlink.id = self.id;
        vlink.alter_id = self.aid;
        vlink.network = self.net;
        vlink.header_type = self.r#type;
        vlink.request_host = self.host;
        vlink.path = self.path;
        vlink.stream_security = self.tls;

        vlink
    }
}

#[allow(dead_code)]
pub async fn fetch(url: &str) -> Result<Vec<VLink>, Box<dyn std::error::Error>> {
    let mut v = Vec::new();

    let text = reqwest::Client::builder()
        .user_agent("V2rayMaid")
        .build()?
        .get(url)
        .send()
        .await?
        .bytes()
        .await?;
    let text = String::from_utf8(base64::decode(&text)?)?;

    for line in text.lines() {
        if !line.starts_with("vmess://") {
            continue;
        }
        let buf = base64::decode(&line[8..])?;
        let json = serde_json::from_slice::<VmessShare>(&buf);
        if let Ok(vmess) = json {
            v.push(vmess.into());
        } else {
            println!("{:?}", String::from_utf8(buf.clone()));
            println!("反序列化失败");
            continue;
        }
    }
    Ok(v)
}
