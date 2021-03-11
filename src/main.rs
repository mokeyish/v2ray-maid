mod ping;
mod settings;
mod subscription;
mod utils;
mod v2ray_ctl;
mod v2ray_object;
mod v2ray_template;
mod vlink;
use crate::v2ray_object::V2rayObject;
use log::info;
use std::option::Option::Some;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut settings = settings::load_settings("v2ray-maid.json").expect("load settings failed.");
    let log_level =
        log::LevelFilter::from_str(settings.loglevel.as_str()).unwrap_or(log::LevelFilter::Info);
    log::set_max_level(log_level);
    log::set_logger(&LOGGER).unwrap();

    {
        let mut ctl = v2ray_ctl::init(settings.program.as_str());
        let mut subscriptions = subscription::fetch(settings.sub_url.as_str()).await?;
        for vlink in &mut subscriptions {
            for proxy in &mut settings.proxies {
                if regex::Regex::new(proxy.selector.as_str())
                    .map(|re| re.is_match(vlink.remarks.as_str()))
                    .unwrap_or(false)
                {
                    let v2ray_json = serde_json::to_string(&vlink.gen_full(None))?;
                    ctl.set_conf(v2ray_json.as_str())?;
                    ctl.start()?;
                    vlink.latency = ping::google(
                        vlink.remarks.as_str(),
                        Some("http://127.0.0.1:21954"),
                        settings.ping_times.unwrap_or(5),
                    )
                    .await;
                    ctl.stop()?;
                    if vlink.latency > 0 {
                        proxy.add_link(vlink.clone())
                    }
                }
            }
        }

        for proxy in &mut settings.proxies {
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

    for proxy in &settings.proxies {
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
