pub mod server;

use aes::Aes128;
use base64::{engine, Engine as _};
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
use chrono::Local;
use clap::{Parser, Subcommand};
use google_authenticator::GoogleAuthenticator;
use log::info;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Read;
use std::io::Write;
use std::net::SocketAddrV4;
use std::path::{Path, PathBuf};
type Aes128Cbc = Cbc<Aes128, Pkcs7>;

pub const KEY: &[u8; 16] = b"53CA391E08237401";
pub const IV: &[u8; 16] = b"F054DD0F4E884616";

#[derive(Parser, Clone, Debug)]
#[command(name = "Passwd Parser")]
#[command(author = "Victor <bobmialvv@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "密码助手", long_about = None)]
pub struct Passwd {
    #[arg(short = 'f', long, help = "密码文件存放位置")]
    pub file: PathBuf,

    #[command(subcommand)]
    pub commands: Commands,
}

impl Passwd {
    pub fn new(file: PathBuf) -> Passwd {
        Passwd {
            file: file,
            commands: Commands::StartServer {
                address: "127.0.0.1".to_string(),
                port: 8084,
            },
        }
    }
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    Decode,
    Encode,
    
    Service {
        #[arg(short = 'l', long, default_value = "", help = "日志保存路径")]
        log_path: String,
    },
    StartServer {
        #[arg(short = 'a', long, default_value = "127.0.0.1", help = "监听地址")]
        address: String,

        #[arg(short = 'p', long, default_value = "8083", help = "监听端口")]
        port: u32,
    },
}

impl Passwd {
    pub fn get_socket(&self) -> SocketAddrV4 {
        match &self.commands {
            Commands::StartServer { address, port } => {
                let address = format!("{}:{}", address, port);
                address.parse::<SocketAddrV4>().unwrap()
            }
            _ => "127.0.0.1:8083".parse::<SocketAddrV4>().unwrap(),
        }
    }

    pub fn get_passwd(&self, host: &str, _account: Option<&str>) -> Result<String, String> {
        let mut passwd_string = self.get_content()?;
        if self.is_encode(&passwd_string) {
            passwd_string = passwd_string.trim_matches('#').to_string();
            passwd_string = self.decode(&passwd_string)?;
        }
        let url_regex = Regex::new(r"http(s)?://([\w\-\.]+)").unwrap();
        let comment_regex = Regex::new(r"(?m)^#[-\S# \r]+\n").unwrap();
        passwd_string = comment_regex.replace_all(&passwd_string, "").to_string();
        let passwds = passwd_string
            .split("\n\n")
            .filter(|item| !item.is_empty())
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut passwd_account = String::from("");
        for passwd_str in passwds {
            if passwd_str.find(host).is_some() {
                passwd_account = passwd_str;
                break;
            }
        }

        let mut hosts = Vec::<String>::new();
        let mut accounts = Vec::<Account>::new();
        let account_regex = Regex::new(r"(?<start>[\d]+)\-(?<end>[\d]+)").unwrap();

        let username_parse = |mut username: String| -> Vec<String> {
            let mut usernames = Vec::<String>::new();
            let result = account_regex.captures(&username);
            if let Some(captures) = result {
                let mut max_len = 0;
                let start = captures
                    .name("start")
                    .map_or(0, |m| m.as_str().parse::<u32>().unwrap_or_default());
                let end = captures.name("end").map_or(0, |m| {
                    let regex_match = m.as_str();
                    max_len = regex_match.len();
                    regex_match.parse::<u32>().unwrap_or_default() + 1
                });
                if start > end {
                    usernames.push(username);
                } else {
                    username = account_regex.replace(&username, "").to_string();
                    for idx in start..end {
                        usernames.push(format!("{}{:0max$}", username, idx, max = max_len));
                    }
                }
            } else {
                usernames.push(username)
            }
            usernames
        };

        let passwd_accounts = passwd_account
            .split("\n")
            .map(|mut item| {
                let result = url_regex.captures(item);
                match result {
                    Some(captures) => {
                        let regex_match = captures.get(2);
                        if let Some(capture_host) = regex_match {
                            hosts.push(capture_host.as_str().to_string());
                        } else {
                            hosts.push(item.to_string());
                        }

                        item = "";
                        item.to_string()
                    }
                    None => item.trim().to_string(),
                }
            })
            .filter(|item| !item.is_empty())
            .collect::<Vec<String>>()
            .split(|s| s == "-")
            .map(|s| s.to_vec())
            .collect::<Vec<Vec<String>>>();

        for account in passwd_accounts {
            match &account[..] {
                [username, passswd, auther @ ..] => {
                    let _2fa = auther.get(0);
                    let mut code = "".to_string();
                    if _2fa.is_some() {
                        let auth = GoogleAuthenticator::new();
                        let result = auth.get_code(&_2fa.unwrap(), 0);
                        if result.is_ok() {
                            code = result.unwrap();
                        }
                    }
                    for username in username_parse(username.to_string()).iter() {
                        let account = Account::new(
                            username.clone(),
                            passswd.clone(),
                            "".to_string(),
                            code.clone(),
                        );
                        accounts.push(account);
                    }
                }
                _ => {}
            }
        }
        let response =
            serde_json::to_string(&Accounts::new(hosts, accounts)).map_err(|e| e.to_string())?;

        info!("response:{}", response);
        Ok(self.encrypt(&response)?)
    }

    pub fn encrypt(&self, message: &str) -> Result<String, String> {
        let plaintext = message.as_bytes();
        let slice = plaintext.to_owned();
        let buffer = slice.as_slice();
        let cipher = Aes128Cbc::new_from_slices(KEY, IV).unwrap();
        let ciphertext = cipher.encrypt_vec(buffer);
        let encoded = engine::general_purpose::STANDARD_NO_PAD.encode(ciphertext);
        info!("AES-128-CBC Base64 encode:{}", encoded);
        Ok(encoded)
    }

    pub fn get_content(&self) -> Result<String, String> {
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .open(self.file.clone())
            .map_err(|e| e.to_string())?;
        let mut reader = std::io::BufReader::new(&mut file);
        let mut contents = String::new();
        let _ = reader.read_to_string(&mut contents);
        Ok(contents)
    }

    pub fn decode(&self, base64_encode_message: &str) -> Result<String, String> {
        let buffer = engine::general_purpose::STANDARD_NO_PAD
            .decode(base64_encode_message)
            .map_err(|e| e.to_string())?;
        let cipher = Aes128Cbc::new_from_slices(KEY, IV).unwrap();
        let ciphertext = cipher.decrypt_vec(&buffer).map_err(|e| e.to_string())?;

        let encode_passwd = String::from_utf8_lossy(&ciphertext).to_string();
        Ok(encode_passwd)
    }

    pub fn is_encode(&self, message: &str) -> bool {
        message.starts_with("###")
    }

    pub fn encode_tofile(&self) -> Result<bool, String> {
        let message = self.get_content()?;
        if self.is_encode(&message) {
            return Ok(true);
        }
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .open(self.file.clone())
            .map_err(|e| e.to_string())?;
        let mut writer = std::io::BufWriter::new(file);
        let txt = format!("###{}", self.encrypt(&message)?);
        let _ = writer.write_all(txt.as_bytes());
        Ok(true)
    }

    pub fn decode_tofile(&self) -> Result<bool, String> {
        let mut message = self.get_content()?;
        if !self.is_encode(&message) {
            return Ok(true);
        }
        message = message.trim_matches('#').to_string();
        info!("message:{}", message);
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .open(self.file.clone())
            .map_err(|e| e.to_string())?;
        let mut writer = std::io::BufWriter::new(file);
        let txt = self.decode(&message)?;
        let _ = writer.write_all(txt.as_bytes());
        Ok(true)
    }

    pub fn service(&self) -> Result<bool, String> {
        let service = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>YB.wdbox</string>
    <key>ProgramArguments</key>
      <array>
          <string>{program}</string>
          <string>-f</string>
          <string>{passwd_file}</string>
          <string>start-server</string>
      </array>
    <key>RunAtLoad</key>
    <true/>
    <key>OnDemand</key>
    <true/>
    <key>LaunchOnlyOnce</key>
    <true/>
    <key>StandardErrorPath</key>
    <string>{log_file}</string>
    <key>StandardOutPath</key>
    <string>{log_file}</string>
</dict>
</plist>"#;
        let program = std::env::current_exe().map_err(|e| e.to_string())?;

        let default_log_path = || -> Result<String, String> {
            let home = std::env::var("HOME").map_err(|e| e.to_string())?;
            let filename = program
                .file_name()
                .ok_or("无法获取运行程序名字")?
                .to_str()
                .ok_or("无法获取运行程序名字")?;

            let path = PathBuf::from(home)
                .join(format!(".{}", filename))
                .join("log.txt");
            info!("{}", filename);

            Ok(path
                .to_str()
                .ok_or("无法获取文件默认的日志路径")?
                .to_string())
        };

        let log_file = match &self.commands {
            Commands::Service { log_path } => {
                if log_path.is_empty() {
                    default_log_path()?
                } else {
                    log_path.clone()
                }
            }
            _ => default_log_path()?,
        };
        let mut log_file = Path::new(&log_file).to_owned();
        if !log_file.exists() {
            let _ = fs::create_dir_all(log_file.parent().ok_or("无法获取日志路径")?);
            File::create(&log_file).map_err(|e|e.to_string())?;
        }
     
        if !log_file.is_absolute() {
            log_file = std::fs::canonicalize(log_file).map_err(|e| e.to_string())?;
        }
    
        info!("log path:{:?}", log_file);
        let mut passwd_file = self.file.clone();
        let path = passwd_file.as_path();
        if !path.is_file() {
            return Err("文件不存在".to_string());
        }

        passwd_file = std::fs::canonicalize(&passwd_file).map_err(|e| e.to_string())?;

        let service = service
            .replace(
                "{program}",
                program.to_str().ok_or("无法获取当前程序名称！")?,
            )
            .replace(
                "{passwd_file}",
                passwd_file.to_str().ok_or("获取密码保存文件失败！")?,
            )
            .replace("{log_file}", log_file.to_str().ok_or("获取日志地址失败!")?);
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open("wdbox.plist")
            .map_err(|e| e.to_string())?;
        let mut writer = std::io::BufWriter::new(file);

        let _ = writer.write_all(service.as_bytes());
        Ok(true)
    }
}

#[derive(Deserialize, Debug)]
pub struct Host {
    host: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Accounts {
    hosts: Vec<String>,
    passwds: Vec<Account>,
}

impl Accounts {
    pub fn new(hosts: Vec<String>, passwds: Vec<Account>) -> Self {
        Accounts { hosts, passwds }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Account {
    username: String,
    password: String,
    #[allow(dead_code)]
    #[serde(skip)]
    auther: String,
    code: String,
    time: String,
}

impl Account {
    pub fn new(username: String, password: String, auther: String, code: String) -> Self {
        Account {
            username,
            password,
            auther,
            code,
            time: Local::now().to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use log::info;

    use super::Passwd;
    use std::{env, path::PathBuf};

    fn passwd() -> Passwd {
        let result = env::var("HOME");
        assert!(result.is_ok());
        let home_path = result.unwrap();
        let filepath = "";
        assert!(!filepath.is_empty(),"请设置相关密码文件路径进行测试！");
        let file = PathBuf::from(home_path).join("");
        let cli = Passwd::new(file);
        cli
    }

    fn init_log() {
        std::env::set_var("RUST_LOG", "debug");
        std::env::set_var("RUST_BACKTRACE", "1");
        let _ = env_logger::try_init();
    }

    #[test]
    fn get_passwd() {
        init_log();

        let cli = passwd();
        let url = cli.get_passwd("blog.csdn.net", None);

        assert!(url.is_ok());
        let content = url.unwrap();
    }

    #[test]
    fn encrypt() {
        init_log();
        let message = r#"{\"hosts\":[\"blog.csdn.net\"],\"passwds\":[{\"username\":\"victor\",\"password\":\"abcd123456\",\"auther\":\"\",\"code\":\"\"}]}"#;
        let cli = passwd();
        let encode = cli.encrypt(message);
        assert!(encode.is_ok());
        info!("encode data:{}", encode.unwrap());
    }

    #[test]
    fn decode() {
        init_log();
        let base64_encode_message = String::from("2w6I/meEiLZMUa8g6NDhSJYBYwSTLKYWV6ObnXsh90hXLKa24VhtjR5VW4AEq6tJ9IMZHtKUhBr/8D9HdWSWfOnNVSL1Z7YsUigi1D1Q60ojEdnAgpcZ5+nY7bwzbu4ECzzqrwkhsZJP3NDzia9sskZdgIv7DyBBB+dyUQ9ylz7QJRuMwqkWtzlL3WIwJSkj");
        let cli = passwd();
        let url = cli.decode(&base64_encode_message);

        assert!(url.is_ok());
        info!("{:?}", url.unwrap())
    }
}
