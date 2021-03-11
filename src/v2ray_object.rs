use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct V2rayObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log: Option<LogObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns: Option<dns::DnsObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbounds: Option<Vec<inbound::InboundObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbounds: Option<Vec<outbound::OutboundObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing: Option<routing::RoutingObject>,
}

pub mod dns {
    use super::*;

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct DnsObject {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub servers: Option<Vec<ServerObject>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub hosts: Option<HashMap<String, String>>,
        #[serde(rename = "clientIp", skip_serializing_if = "Option::is_none")]
        pub client_ip: Option<String>,
        #[serde(rename = "disableCache", skip_serializing_if = "Option::is_none")]
        pub disable_cache: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tag: Option<String>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct ServerObject {
        pub address: String,
        pub port: Option<super::Port>,
        #[serde(rename = "clientIp", skip_serializing_if = "Option::is_none")]
        pub client_ip: Option<String>,
        pub domains: Option<Vec<String>>,
        #[serde(rename = "exceptIps", skip_serializing_if = "Option::is_none")]
        pub except_ips: Option<Vec<String>>,
    }
}

pub mod routing {
    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum DomainStrategy {
        AsIs,
        IPIfNonMatch,
        IPOnDemand,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub enum DomainMatcher {
        Linear,
        Hybrid,
    }
    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct RoutingObject {
        #[serde(rename = "domainStrategy", skip_serializing_if = "Option::is_none")]
        pub domain_strategy: Option<DomainStrategy>,
        #[serde(rename = "domainMatcher", skip_serializing_if = "Option::is_none")]
        pub domain_matcher: Option<DomainMatcher>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub balancers: Option<Vec<BalancerObject>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rules: Option<Vec<RuleObject>>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct RuleObject {
        #[serde(rename = "domainMatcher", skip_serializing_if = "Option::is_none")]
        pub domain_matcher: Option<DomainMatcher>,
        pub r#type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub domain: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ip: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub port: Option<super::Port>,
        #[serde(rename = "sourcePort", skip_serializing_if = "Option::is_none")]
        pub source_port: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub network: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub source: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub user: Option<Vec<String>>,
        #[serde(rename = "inboundTag", skip_serializing_if = "Option::is_none")]
        pub inbound_tag: Option<Vec<String>>,
        #[serde(rename = "outboundTag", skip_serializing_if = "Option::is_none")]
        pub outbound_tag: Option<String>,
        #[serde(rename = "balancerTag", skip_serializing_if = "Option::is_none")]
        pub balancer_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attrs: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub protocol: Option<Vec<String>>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct BalancerObject {
        pub tag: String,
        pub selector: Vec<String>,
    }
}


pub mod inbound {
    use super::*;

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct InboundObject {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub listen: Option<String>,
        pub port: super::Port,
        pub protocol: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub settings: Option<InboundConfigurationObject>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tag: Option<String>,
        #[serde(rename = "streamSettings", skip_serializing_if = "Option::is_none")]
        pub stream_settings: Option<super::stream_settings::StreamSettingsObject>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sniffing: Option<SniffingObject>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub allocate: Option<AllocateObject>,
    }

    #[derive(Default, Debug, Clone, Deserialize, Serialize)]
    pub struct InboundConfigurationObject {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub timeout: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub auth: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub accounts: Option<Vec<AccountObject>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub udp: Option<bool>,
        #[serde(rename = "userLevel", skip_serializing_if = "Option::is_none")]
        pub user_level: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub port: Option<u16>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub network: Option<String>,
    }

    #[derive(Default, Debug, Clone, Deserialize, Serialize)]
    pub struct AccountObject {
        pub user: String,
        pub pass: String
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct SniffingObject {
        pub enabled: bool,
        #[serde(rename = "destOverride")]
        pub dest_override: Option<Vec<String>>,
        #[serde(rename = "metadata_only")]
        pub metadata_only: bool,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct AllocateObject {
        pub strategy: Option<Strategy>,
        pub refresh: i32,
        pub concurrency: i32,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub enum Strategy {
        Always,
        Random,
    }
}

pub mod outbound {
    use super::*;

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct OutboundObject {
        #[serde(rename = "sendThrough", skip_serializing_if = "Option::is_none")]
        pub send_through: Option<String>,
        pub tag: String,
        pub protocol: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub settings: Option<OutboundConfigurationObject>,
        #[serde(rename = "streamSettings", skip_serializing_if = "Option::is_none")]
        pub stream_settings: Option<super::stream_settings::StreamSettingsObject>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub mux: Option<MuxObject>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum DomainStrategy {
        AsIs,
        UseIP,
        UseIPv4,
        UseIPv6
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct OutboundConfigurationObject {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub vnext: Option<Vec<super::ServerObject>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub servers: Option<Vec<super::SocksServerObject>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub response: Option<super::Response>,
        #[serde(rename = "domainStrategy", skip_serializing_if = "Option::is_none")]
        pub domain_strategy: Option<DomainStrategy>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct ProxySettings {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tag: Option<String>,
        #[serde(rename = "transportLayer", skip_serializing_if = "Option::is_none")]
        pub transport_layer: Option<bool>
    }

    #[derive(Default, Debug, Clone, Deserialize, Serialize)]
    pub struct MuxObject {
        pub enabled: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub concurrency: Option<i32>,
    }
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerObject {
    pub address: String,
    pub port: Port,
    pub users: Vec<UserObject>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserObject {
    pub id: String,
    #[serde(rename = "alterId")]
    pub alter_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SocksServerObject {
    pub address: String,
    pub port: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<SocksUserObject>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SocksUserObject {
    pub user: String,
    pub pass: String,
    pub level: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Response {
    pub r#type: String,
}

pub mod stream_settings {
    use super::*;


    #[derive(Default, Debug, Clone, Deserialize, Serialize)]
    pub struct StreamSettingsObject {
        pub network: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub security: Option<String>,
        #[serde(rename = "tlsSettings", skip_serializing_if = "Option::is_none")]
        pub tls_settings: Option<TLSObject>,
        #[serde(rename = "tcpSettings", skip_serializing_if = "Option::is_none")]
        pub tcp_settings: Option<TcpObject>,
        #[serde(rename = "kcpSettings", skip_serializing_if = "Option::is_none")]
        pub kcp_settings: Option<KcpObject>,
        #[serde(rename = "wsSettings", skip_serializing_if = "Option::is_none")]
        pub ws_settings: Option<WebSocketObject>,
        #[serde(rename = "httpSettings", skip_serializing_if = "Option::is_none")]
        pub http_settings: Option<HttpObject>,
        #[serde(rename = "quicSettings", skip_serializing_if = "Option::is_none")]
        pub quic_settings: Option<QuicObject>,
        #[serde(rename = "dsSettings", skip_serializing_if = "Option::is_none")]
        pub ds_settings: Option<DomainSocketObject>,
        #[serde(rename = "sockopt", skip_serializing_if = "Option::is_none")]
        pub sock_opt: Option<SockOpt>
    }

    #[derive(Default, Debug, Clone, Deserialize, Serialize)]
    pub struct TLSObject {
        #[serde(rename = "serverName", skip_serializing_if = "Option::is_none")]
        pub server_name: Option<String>,
        #[serde(rename = "allowInsecure", skip_serializing_if = "Option::is_none")]
        pub allow_insecure: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub alpn: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub certificates: Option<Vec<CertificateObject>>,
        #[serde(rename = "disableSystemRoot", skip_serializing_if = "Option::is_none")]
        pub disable_system_root: Option<bool>
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct CertificateObject {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub usage: Option<String>,
        #[serde(rename = "certificateFile", skip_serializing_if = "Option::is_none")]
        pub certificate_file: Option<String>,
        #[serde(rename = "keyFile", skip_serializing_if = "Option::is_none")]
        pub key_file: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub certificate: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub key: Option<Vec<String>>
    }


    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct TcpObject {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub accept_proxy_protocol: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub header: Option<PseudoHeaderObject>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    #[serde(tag = "type", rename_all = "camelCase")]
    pub enum PseudoHeaderObject {
        None,
        Http {
            request: Option<HttpRequestObject>,
            response: Option<HttpResponseObject>
        }
    }

    #[derive(Default, Debug, Clone, Deserialize, Serialize)]
    pub struct HttpRequestObject {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub method: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub path: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub headers: Option<HashMap<String, HttpHeaderValueObject>>
    }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum HttpHeaderValueObject {
        String(String),
        Array(Vec<String>)
    }

    #[derive(Default, Debug, Clone, Deserialize, Serialize)]
    pub struct HttpResponseObject {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub reason: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub headers: Option<HashMap<String, HttpHeaderValueObject>>
    }

    #[derive(Default, Debug, Clone, Deserialize, Serialize)]
    pub struct KcpObject {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub mtu: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tti: Option<i32>,
        #[serde(rename = "uplinkCapacity", skip_serializing_if = "Option::is_none")]
        pub uplink_capacity: Option<i32>,
        #[serde(rename = "downlinkCapacity", skip_serializing_if = "Option::is_none")]
        pub downlink_capacity: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub congestion: Option<bool>,
        #[serde(rename = "readBufferSize", skip_serializing_if = "Option::is_none")]
        pub read_buffer_size: Option<i32>,
        #[serde(rename = "writeBufferSize", skip_serializing_if = "Option::is_none")]
        pub write_buffer_size: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub header: Option<HeaderObject>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub seed: Option<String>
    }

    #[derive(Default, Debug, Clone, Deserialize, Serialize)]
    pub struct HeaderObject {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<String>
    }


    #[derive(Default, Debug, Clone, Deserialize, Serialize)]
    pub struct WebSocketObject {
        #[serde(rename = "acceptProxyProtocol", skip_serializing_if = "Option::is_none")]
        pub accept_proxy_protocol: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub path: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub headers: Option<HashMap<String, String>>,
    }

    #[derive(Default, Debug, Clone, Deserialize, Serialize)]
    pub struct HttpObject {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub host: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub path: Option<String>,
    }

    #[derive(Default, Debug, Clone, Deserialize, Serialize)]
    pub struct QuicObject {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub security: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub header: Option<HeaderObject>,
    }

    #[derive(Default, Debug, Clone, Deserialize, Serialize)]
    pub struct DomainSocketObject {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub path: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#abstract: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub padding: Option<bool>
    }

    #[derive(Default, Debug, Clone, Deserialize, Serialize)]
    pub struct SockOpt {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub mark: Option<i32>,
        #[serde(rename = "tcpFastOpen", skip_serializing_if = "Option::is_none")]
        pub tcp_fast_open: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tproxy: Option<bool>
    }
}



#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LogObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loglevel: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Port {
    Int(u16),
    String(String),
}



