use crate::NOTION_RSS_PATH;
use argh::FromArgs;
use notion_sdk::database::id::DatabaseId;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone, FromArgs)]
#[argh(description = "notion-rss")]
pub struct NotionConfig {
    /// the notion api token
    #[argh(option)]
    pub notion_token: Option<String>,
    /// the source database id
    #[argh(option)]
    pub source_id: Option<DatabaseId>,
    /// the archive database id
    #[argh(option)]
    pub archive_id: Option<DatabaseId>,
    /// read the feed from the file
    #[argh(option)]
    #[serde(skip)]
    pub file: Option<PathBuf>,
    /// read the config from the file
    #[argh(option, short = 'c')]
    #[serde(skip)]
    pub config: Option<PathBuf>,
    /// proxy to use for requests (ex:[http(s)|socks5(h)]://host:port)
    #[argh(option)]
    #[serde(default)]
    pub proxy: Option<String>,
    /// set request timeout.
    #[argh(option, default = "default_timeout()")]
    #[serde(default = "default_timeout")]
    pub timeout: u64,
    /// update self
    #[argh(switch)]
    #[serde(skip)]
    pub update: bool,
    /// deleted old archive
    #[argh(switch)]
    #[serde(skip)]
    pub deleted: bool,
    /// number of concurrent threads.
    #[argh(option, default = "default_thread()")]
    #[serde(default = "default_thread")]
    pub thread: u32,
    /// send results to webhook server (ex:https://host:port/webhook)
    #[argh(option)]
    #[serde(default)]
    pub webhook: Option<String>,
    /// start a web API service (ex:127.0.0.1:8080)
    #[argh(option)]
    #[serde(default)]
    pub api_server: Option<String>,
    /// api Router authentication
    #[argh(option, default = "default_token()")]
    #[serde(default = "default_token")]
    pub token: String,
    /// api background service
    #[argh(switch)]
    #[serde(default)]
    pub daemon: bool,
    /// cli mode
    #[argh(switch)]
    #[serde(skip)]
    pub cli: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct YamlConfig {
    config: NotionConfig,
}

fn default_thread() -> u32 {
    5_u32
}

fn default_token() -> String {
    let hasher = openssl::hash::Hasher::new(openssl::hash::MessageDigest::md5());
    if let Ok(mut h) = hasher {
        let mut test_bytes = vec![0u8; 32];
        openssl::rand::rand_bytes(&mut test_bytes).unwrap_or_default();
        h.update(&test_bytes).unwrap_or_default();
        if let Ok(bytes) = h.finish() {
            let hex: String = bytes
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<Vec<String>>()
                .join("");
            return hex;
        }
    }
    String::new()
}

fn default_timeout() -> u64 {
    15
}

impl NotionConfig {
    pub fn save(&self) -> String {
        let config_path = NOTION_RSS_PATH.join("config.yaml");
        match File::create(config_path) {
            Ok(out) => {
                if let Err(err) = serde_yaml::to_writer(
                    out,
                    &YamlConfig {
                        config: self.clone(),
                    },
                ) {
                    err.to_string()
                } else {
                    "Update success".to_string()
                }
            }
            Err(err) => err.to_string(),
        }
    }
    pub fn reload() -> NotionConfig {
        let config_path = NOTION_RSS_PATH.join("config.yaml");
        if let Ok(file) = File::open(config_path) {
            if let Ok(config) = serde_yaml::from_reader::<_, YamlConfig>(&file) {
                return config.config;
            };
        }
        NotionConfig::default()
    }
    // fn merge(self, config: NotionConfig) -> Self {
    //     Self {
    //         notion_token: self.notion_token.or(config.notion_token),
    //         source_id: self.source_id.or(config.source_id),
    //         archive_id: self.archive_id.or(config.archive_id),
    //         file: self.file,
    //         config: None,
    //         proxy: self.proxy.or(config.proxy),
    //         timeout: self.timeout | config.timeout,
    //         update: self.update | config.update,
    //         deleted: self.deleted,
    //         thread: self.thread | config.thread,
    //         webhook: self.webhook.or(config.webhook),
    //         api_server: self.api_server.or(config.api_server),
    //         token: if self.token.is_empty() {
    //             config.token
    //         } else {
    //             self.token
    //         },
    //         daemon: self.daemon | config.daemon,
    //         cli: false,
    //     }
    // }
}

impl Default for NotionConfig {
    fn default() -> Self {
        let mut default: NotionConfig = argh::from_env();
        let mut config_path = NOTION_RSS_PATH.join("config.yaml");
        if let Some(config_file) = default.config.clone() {
            if config_file.to_str() != Some("default") {
                config_path = config_file
            }
        }
        if config_path.exists() {
            if let Ok(file) = File::open(&config_path) {
                match serde_yaml::from_reader::<_, YamlConfig>(&file) {
                    Ok(config) => {
                        if default.cli && default.config.is_some() {
                            default = config.config.clone();
                        }
                        if !default.cli {
                            default = config.config;
                        }
                    }
                    Err(err) => {
                        println!("Failed to read configuration file: {}", err);
                        std::process::exit(0);
                    }
                };
            }
        }
        default
    }
}
