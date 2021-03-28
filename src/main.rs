#![feature(async_closure)]

mod ping;
mod settings;
mod subscription;
mod utils;
mod v2ray_ctl;
mod v2ray_object;
mod v2ray_template;
mod vlink;

use crate::settings::AppSettings;
use crate::utils::pick_free_tcp_port;
use crate::v2ray_ctl::V2rayApp;
use crate::v2ray_object::{Port, V2rayObject};
use crate::vlink::VLink;
use log::info;
use std::option::Option::Some;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        r#"
     __   _____ ___    ___   __    __  __   _   ___ ___
     \ \ / /_  ) _ \  /_\ \ / /   |  \/  | /_\ |_ _|   \
      \ V / / /|   / / _ \ V /    | |\/| |/ _ \ | || |) |
       \_/ /___|_|_\/_/ \_\_|     |_|  |_/_/ \_\___|___/
    "#
    );
    let settings: Arc<AppSettings> = settings::load_settings("v2ray-maid.json")
        .expect("load settings failed.")
        .into();
    let log_level =
        log::LevelFilter::from_str(settings.loglevel.as_str()).unwrap_or(log::LevelFilter::Info);
    log::set_max_level(log_level);
    log::set_logger(&LOGGER).unwrap();
    let ctl: Arc<V2rayApp> = v2ray_ctl::init(settings.program.as_str()).into();

    let mut proxies = settings.proxies.clone().unwrap_or(Vec::new());

    {
        let subs = subscription::fetch(settings.sub_url.as_str()).await?;
        let subs = parallel_test_latency(subs, &ctl, &settings).await;
        for proxy in &mut proxies {
            let regex = regex::Regex::new(proxy.selector.as_str());

            for vlink in &subs {
                if regex
                    .as_ref()
                    .map(|re| re.is_match(vlink.remarks.as_str()))
                    .unwrap_or(false)
                {
                    proxy.vlinks.push(vlink.clone());
                }
            }
        }

        for proxy in &mut proxies {
            proxy.vlinks.sort_by(|a, b| {
                if a.latency == b.latency {
                    std::cmp::Ordering::Equal
                } else if a.latency > b.latency {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            })
        }
    }

    for proxy in &proxies {
        let vlink = proxy.vlinks.first();
        if let Some(v) = vlink {
            info!(
                "『{}』 最快的服务器是 『{}』，延迟 {} ms",
                proxy.selector, v.remarks, v.latency
            );

            let tag = proxy.tag.as_ref().map(|s| s.as_str()).unwrap_or("proxy");

            let mut v2ray_object = serde_json::from_slice::<V2rayObject>(
                std::fs::read(proxy.target_file.as_str())?.as_ref(),
            )?;

            let outbounds = if let Some(outbounds) = &mut v2ray_object.outbounds {
                outbounds.retain(|o| !o.tag.as_str().starts_with(tag));
                outbounds
            } else {
                v2ray_object.outbounds = Some(Vec::new());
                v2ray_object.outbounds.as_mut().unwrap()
            };

            let limit = proxy.limit.unwrap_or(1);
            if limit == 1 {
                let outbound = v.gen_outbound(&tag, None);
                outbounds.insert(0, outbound);
            } else {
                for (i, v) in proxy.vlinks.iter().enumerate() {
                    let outbound = v.gen_outbound(format!("{}_{}", tag, i).as_str(), None);
                    outbounds.insert(0, outbound);
                    if i == limit {
                        break;
                    }
                }
            }

            std::fs::write(
                proxy.target_file.as_str(),
                serde_json::to_string_pretty(&v2ray_object)?,
            )?;
            info!(
                "已更新配置：{}，使用服务器『{}』",
                proxy.target_file.as_str(),
                v.remarks.as_str()
            );
        } else {
            info!("『{}』 没有可用的服务器", proxy.selector);
        }
    }
    Ok(())
}

async fn parallel_test_latency<'a>(
    subs: Vec<VLink>,
    ctl: &Arc<V2rayApp>,
    settings: &Arc<AppSettings>,
) -> Vec<VLink> {
    async fn test_latency(vlink: &VLink, ctl: &V2rayApp, settings: &AppSettings) -> i32 {
        let mut v2ray_config = vlink.gen_full(None);
        let listen_port = pick_free_tcp_port();
        let inbound = v2ray_config.inbounds.as_mut().unwrap().first_mut().unwrap();
        inbound.port = Port::Int(listen_port);
        let v2ray_json = serde_json::to_string(&v2ray_config).unwrap();
        let process = ctl.start(v2ray_json.as_str()).unwrap();
        let latency = ping::google(
            vlink.remarks.as_str(),
            Some(format!("http://127.0.0.1:{}", listen_port).as_str()),
            settings.ping_times.unwrap_or(5),
        )
        .await;
        ctl.stop(process).unwrap();
        latency
    }

    let len = subs.len();

    let in_subs = Arc::new(Mutex::new(subs));
    let out_subs = Arc::new(Mutex::new(Vec::with_capacity(len)));

    let mut threads = Vec::new();
    let concurrency = match settings.concurrency {
        Some(c) => {
            if len < c {
                len
            } else {
                c
            }
        }
        None => 1,
    };
    info!("Find {} servers", len);
    info!("Ping with {} threads", concurrency);
    for _ in 0..concurrency {
        let ctl = ctl.clone();
        let settings = settings.clone();
        let in_subs = in_subs.clone();
        let out_subs = out_subs.clone();
        let thread = tokio::spawn(async move {
            while let Some(mut vlink) = {
                let mut subs = in_subs.lock().unwrap();
                let vlink = subs.pop();
                vlink
            } {
                vlink.latency = test_latency(&vlink, &ctl, &settings).await;
                if vlink.latency > 0 {
                    let mut subs = out_subs.lock().unwrap();
                    subs.push(vlink);
                }
            }
        });
        threads.push(thread)
    }

    for thread in threads {
        thread.await.unwrap()
    }

    let out = out_subs.lock().unwrap();
    out.to_vec()
}

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Debug
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;
