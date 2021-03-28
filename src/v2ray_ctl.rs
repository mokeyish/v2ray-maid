use crate::utils::find_it;
use log::{error, info};
use std::io::BufRead;
use std::path::PathBuf;
use std::process::exit;
use uuid::Uuid;

pub struct V2rayApp {
    program: PathBuf,
    version: String,
}

pub struct V2rayAppProcess {
    child: std::process::Child,
    cfg_path: PathBuf,
}

impl V2rayApp {
    #[inline]
    pub fn init(program: &str) -> Option<V2rayApp> {
        if let Some(p) = find_it(program) {
            info!("V2ray locate at {}.", p.to_str().unwrap());
            let output = std::process::Command::new(p.as_os_str())
                .arg("--version")
                .stdout(std::process::Stdio::piped())
                .output()
                .expect("Failed to get version.");
            let v = String::from_utf8_lossy(output.stdout.as_ref());
            let re = regex::Regex::new(r"V2Ray (?P<ver>\d+\.\d+\.\d+) ").unwrap();
            let ver = &re.captures(v.as_ref()).unwrap()["ver"];
            info!("V2ray's version is {}.", ver);
            Some(Self {
                program: p,
                version: ver.to_string(),
            })
        } else {
            error!("V2ray not found!!!");
            None
        }
    }

    pub fn start(&self, v2ray_json: &str) -> Result<V2rayAppProcess, Box<dyn std::error::Error>> {
        let cfg_path = {
            let mut cfg_path = std::env::temp_dir();
            cfg_path.push(format!(
                "v2ray-maid-running-{}.json",
                Uuid::new_v4().to_simple()
            ));
            std::fs::write(cfg_path.as_path(), v2ray_json.as_bytes())?;
            cfg_path
        };

        let mut child = std::process::Command::new(self.program.as_path())
            .stdout(std::process::Stdio::piped())
            .args(&["-c", cfg_path.to_str().unwrap()])
            .spawn()?;
        let mut reader = std::io::BufReader::new(child.stdout.as_mut().unwrap());
        let mut s = String::new();
        let start_flag = format!("V2Ray {} started", self.version.as_str());
        loop {
            let num = reader.read_line(&mut s)?;
            if num > 0 && s.rfind(start_flag.as_str()).is_some() {
                break;
            }
        }
        Ok(V2rayAppProcess { child, cfg_path })
    }

    pub fn stop(&self, mut process: V2rayAppProcess) -> Result<(), Box<dyn std::error::Error>> {
        process.child.kill()?;
        std::fs::remove_file(process.cfg_path)?;
        Ok(())
    }
}

pub fn init(program: &str) -> V2rayApp {
    match V2rayApp::init(program) {
        Some(t) => t,
        None => exit(0x0100),
    }
}
