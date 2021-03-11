use crate::utils::find_it;
use log::{error, info};
use std::io::BufRead;
use std::path::PathBuf;
use std::process::exit;

pub struct V2rayApp {
    program: PathBuf,
    child: Option<std::process::Child>,
    conf: String,
    version: String,
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
            let mut tmp_dir = std::env::temp_dir();
            tmp_dir.push("v2ray-maid-running.json");
            Some(Self {
                program: p,
                child: None,
                conf: tmp_dir.to_str().unwrap().to_string(),
                version: ver.to_string(),
            })
        } else {
            error!("V2ray not found!!!");
            None
        }
    }

    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut child = std::process::Command::new(self.program.as_path())
            .stdout(std::process::Stdio::piped())
            .args(&["-c", self.conf.as_str()])
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
        self.child = Some(child);
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.child.is_some() {
            let child = self.child.as_mut().unwrap();
            child.kill()?;
            std::fs::remove_file(self.conf.as_str())?;
        }
        Ok(())
    }

    pub fn set_conf(&mut self, v2ray_json: &str) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::write(self.conf.as_str(), v2ray_json.as_bytes())?;
        Ok(())
    }
}

pub fn init(program: &str) -> V2rayApp {
    match V2rayApp::init(program) {
        Some(t) => t,
        None => exit(0x0100),
    }
}
