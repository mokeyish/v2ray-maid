use log::{debug, info};
use std::convert::TryFrom;
use std::time::{Duration, Instant};

pub async fn google(label: &str, proxy: Option<&str>, times: i32) -> i32 {
    let mut client_builder = reqwest::Client::builder().timeout(Duration::from_secs(2));
    if let Some(p) = proxy {
        client_builder = client_builder.proxy(reqwest::Proxy::all(p).unwrap());
    }
    async fn ping(client: &reqwest::Client) -> Result<i32, Box<dyn std::error::Error>> {
        let now = Instant::now();
        let res = client
            .get("https://www.google.com/generate_204")
            .send()
            .await?;
        let elapsed_millis = now.elapsed().as_millis();
        let code = res.status().as_u16();
        if code == 204 || code == 204 && res.content_length().unwrap() == 0 {
            Ok(i32::try_from(elapsed_millis).unwrap())
        } else {
            Ok(-1)
        }
    }

    if let Ok(client) = client_builder.build() {
        let mut total = 0;
        let mut timeout = false;
        for i in 0..times {
            let elapsed_millis = ping(&client).await.unwrap_or(-1);
            debug!("Ping {} {} elapsed {} ms", label, i, elapsed_millis);
            if elapsed_millis == -1 {
                timeout = true;
                debug!("Ping {} skipped.", label);
                break;
            }
            total += elapsed_millis;
        }
        let elapsed_millis = total / times;
        if timeout {
            info!("Ping {} timeout", label);
            -1
        } else {
            info!("Ping {} average elapsed {} ms", label, elapsed_millis);
            elapsed_millis
        }
    } else {
        -1
    }
}
