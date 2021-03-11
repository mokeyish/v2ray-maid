use crate::vlink::VLink;
use std::collections::HashMap;
use crate::v2ray_object::V2rayObject;

const V2RAY_TPL: &str = r#"
{
    "log": {
        "loglevel": "Debug"
    },
    "inbounds": [{
        "port": 21954,
        "listen": "127.0.0.1",
        "protocol": "http"
    }],
    "outbounds": [{
            "tag": "proxy",
            "protocol": "vmess",
            "settings": {
                "vnext": [{
                    "address": "v2ray.cool",
                    "port": 10086,
                    "users": [{
                        "id": "a3482e88-686a-4a58-8126-99c9df64b7bf",
                        "alterId": 64,
                        "security": "auto",
                        "level": 8
                    }]
                }],
                "servers": [{
                    "address": "127.0.0.1",
                    "port": 1234,
                    "users": [{
                        "user": "test user",
                        "pass": "test pass",
                        "level": 0
                    }]
                }]
            },
            "streamSettings": {
                "network": "tcp"
            },
            "mux": {
                "enabled": false
            }
        },
        {
            "protocol": "freedom",
            "tag": "direct"
        }
    ],
    "routing": {
        "domainStrategy": "IPOnDemand",
        "rules": [{
            "type": "field",
            "ip": [
                "geoip:private"
            ],
            "outboundTag": "direct"
        }]
    }
}
"#;

#[derive(Clone)]
pub struct GlobalSettings {
    pub use_mux: Option<bool>,
    pub mux_concurrency: Option<i32>,
}

impl Default for GlobalSettings {
    fn default() -> Self {
        GLOBAL_SETTINGS.clone()
    }
}

const GLOBAL_SETTINGS: GlobalSettings = GlobalSettings {
    use_mux: Some(false),
    mux_concurrency: Some(8),
};

impl VLink {
    pub fn gen_full(&self, global_settings: Option<&GlobalSettings>) -> V2rayObject {
        let global_settings = global_settings.unwrap_or(&GLOBAL_SETTINGS);
        let mut v = serde_json::from_str::<V2rayObject>(V2RAY_TPL).unwrap();
        let outbound = v.outbounds.as_mut().unwrap().get_mut(0).unwrap();
        self.fill_outbound(outbound, "proxy", global_settings);
        v
    }
    pub fn gen_outbound(
        &self,
        tag: &str,
        global_settings: Option<&GlobalSettings>,
    ) -> crate::v2ray_object::outbound::OutboundObject {
        let global_settings = global_settings.unwrap_or(&GLOBAL_SETTINGS);
        let mut outbound = serde_json::from_str::<V2rayObject>(V2RAY_TPL)
            .unwrap()
            .outbounds
            .unwrap()[0]
            .clone();
        self.fill_outbound(&mut outbound, tag, global_settings);
        outbound
    }
    fn fill_outbound(
        &self,
        outbound: &mut crate::v2ray_object::outbound::OutboundObject,
        tag: &str,
        global_settings: &GlobalSettings,
    ) {
        outbound.tag = tag.to_string();
        let mut mux = crate::v2ray_object::outbound::MuxObject::default();
        {
            if let Some(enabled) = global_settings.use_mux {
                mux.enabled = enabled;
            }

            if let Some(concurrency) = global_settings.mux_concurrency {
                mux.concurrency = Some(concurrency);
            }
        }

        match self.protocol.as_str() {
            "vmess" => {
                outbound.settings.as_mut().unwrap().servers = None;
                let vnext = outbound
                    .settings
                    .as_mut()
                    .unwrap()
                    .vnext
                    .as_mut()
                    .unwrap()
                    .get_mut(0)
                    .unwrap();
                vnext.address = self.address.clone();
                vnext.port = crate::v2ray_object::Port::Int( self.port);

                let user = vnext.users.get_mut(0).unwrap();
                user.id = self.id.clone();
                user.alter_id = self.alter_id;
                user.security = Some(self.security.clone());
                user.level = Some(8);

                outbound.mux = Some(mux);

                outbound.stream_settings = Some(self.gen_bound_stream_settings());
            }
            _ => unimplemented!(),
        }
    }

    fn gen_bound_stream_settings(&self) -> crate::v2ray_object::stream_settings::StreamSettingsObject {
        let mut stream_settings = crate::v2ray_object::stream_settings::StreamSettingsObject::default();

        stream_settings.network = Some(self.network.clone());
        stream_settings.security = Some(self.stream_security.clone());

        if let Some(network) = stream_settings.network.as_ref() {

            match network.as_str() {
                "kcp" => {
                    let mut kcp_settings = crate::v2ray_object::stream_settings::KcpObject::default();
                    if kcp_settings.header.is_none() {
                        kcp_settings.header = Some(crate::v2ray_object::stream_settings::HeaderObject::default());
                    }
                    kcp_settings.header.as_mut().unwrap().r#type = Some(self.header_type.clone());
                    stream_settings.kcp_settings = Some(kcp_settings);
                }
                "ws" => {
                    let mut ws_settings = crate::v2ray_object::stream_settings::WebSocketObject::default();
                    let host = self.request_host.trim().to_string();
                    let path = self.path.trim().to_string();
                    if !host.is_empty() {
                        if ws_settings.headers.is_none() {
                            ws_settings.headers = Some(HashMap::new());
                        }
                        ws_settings.headers.as_mut().unwrap().insert("host".to_string(), host.clone());
                    }
                    if !path.is_empty() {
                        ws_settings.path = Some(path);
                    }
                    stream_settings.ws_settings = Some(ws_settings);

                    let mut tls_settings = crate::v2ray_object::stream_settings::TLSObject::default();
                    tls_settings.allow_insecure = Some(false);
                    tls_settings.server_name = Some(host);

                    stream_settings.tls_settings = Some(tls_settings);
                }
                _ => unimplemented!(),
            }
        }

        stream_settings
    }
}
