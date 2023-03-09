use crate::cli::NotionConfig;
use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use encoding_rs::{Encoding, UTF_8};
use mime::Mime;
use notion_sdk::common::file::{ExternalFileObject, FileOrEmojiObject};
use notion_sdk::common::parent::Parent;
use notion_sdk::common::rich_text::{RichText, RichTextCommon, Text};
use notion_sdk::database::date::{DateOrDateTime, DateValue};
use notion_sdk::database::id::DatabaseId;
use notion_sdk::database::properties::{Properties, PropertyValue};
use notion_sdk::database::relation::RelationValue;
use notion_sdk::database::select::SelectedValue;
use notion_sdk::database::Color;
use notion_sdk::pages::id::PageId;
use notion_sdk::pages::{Page, UpdatePage};
use notion_sdk::pagination::Object;
use notion_sdk::search::{DatabaseQuery, FilterCondition, PropertyCondition, RelationCondition};
use notion_sdk::NotionApi;
use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::{header, Client, Proxy, Url};
use select::document::Document;
use select::predicate::Name;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::io::{BufRead, Cursor};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::RwLock;
use std::time::Duration;

pub mod api;
pub mod cli;
pub mod rss;
pub mod tray;
pub mod ui;

static SERVER_LOCK: Lazy<RwLock<bool>> = Lazy::new(|| -> RwLock<bool> { RwLock::new(true) });

static NOTION_RSS_PATH: Lazy<PathBuf> = Lazy::new(|| -> PathBuf {
    let mut config_path = PathBuf::new();
    if let Some(cp) = dirs::config_dir() {
        config_path = cp;
    } else {
        println!("Cannot create config directory{:?}", config_path);
        std::process::exit(0);
    }
    let notion_rss = config_path.join("notion-rss");
    if !notion_rss.is_dir() || !notion_rss.exists() {
        std::fs::create_dir_all(&notion_rss).unwrap_or_default();
    }
    notion_rss
});

static CONFIG: Lazy<RwLock<NotionConfig>> =
    Lazy::new(|| -> RwLock<NotionConfig> { RwLock::new(NotionConfig::default()) });

pub static NOTION_FEED: Lazy<NotionFeed> = Lazy::new(|| -> NotionFeed {
    match NotionFeed::new() {
        Ok(nf) => nf,
        Err(e) => {
            println!("{}", e);
            std::process::exit(1)
        }
    }
});

#[derive(Debug, Clone)]
pub struct NotionFeed {
    pub notion: NotionApi,
    pub client: Client,
    pub proxy_client: Client,
    pub config: NotionConfig,
    pub notion_token: String,
    pub source_id: DatabaseId,
    pub archive_id: DatabaseId,
}

impl NotionFeed {
    fn new() -> Result<Self> {
        let config = CONFIG.read().unwrap().clone();
        let token = match config.notion_token.clone() {
            None => {
                return Err(anyhow!("invalid token"));
            }
            Some(token) => token,
        };
        let source_id = match config.source_id.clone() {
            None => {
                return Err(anyhow!("invalid source_id"));
            }
            Some(token) => token,
        };
        let archive_id = match config.archive_id.clone() {
            None => {
                return Err(anyhow!("invalid archive_id"));
            }
            Some(token) => token,
        };
        let notion = NotionApi::new(token.clone())?;
        let mut headers = header::HeaderMap::new();
        let ua = "Mozilla/5.0 (X11; Linux x86_64; rv:94.0) Gecko/20100101 Firefox/94.0";
        headers.insert(header::USER_AGENT, header::HeaderValue::from_static(ua));
        let client_builder = || {
            Client::builder()
                .pool_max_idle_per_host(0)
                .danger_accept_invalid_certs(true)
                .danger_accept_invalid_hostnames(true)
                .default_headers(headers.clone())
                .timeout(Duration::new(config.timeout, 0))
        };
        let client = client_builder().build()?;
        if let Some(p) = config.proxy.clone() {
            return match Url::parse(&p) {
                Ok(p) => {
                    let proxy_client = client_builder()
                        .proxy(Proxy::custom(move |_| Some(p.clone())))
                        .build()?;
                    Ok(Self {
                        notion,
                        client,
                        proxy_client,
                        config,
                        notion_token: token,
                        source_id,
                        archive_id,
                    })
                }
                Err(e) => Err(anyhow!(e)),
            };
        }
        Ok(Self {
            notion,
            client: client.clone(),
            proxy_client: client,
            config,
            notion_token: token,
            source_id,
            archive_id,
        })
    }
    pub fn client(&self, proxy: bool) -> Client {
        if proxy {
            self.proxy_client.clone()
        } else {
            self.client.clone()
        }
    }
}

static RE_COMPILE_BY_SIZE: Lazy<Regex> =
    Lazy::new(|| -> Regex { Regex::new(r#"(?im)-\d{1,3}x\d{1,3}"#).expect("RE_COMPILE_BY_SIZE") });
static RE_TITLE: Lazy<Regex> = Lazy::new(|| -> Regex {
    Regex::new(r#"(?im)<title>(?P<title>.*?)</title>"#).expect("RE_TITLE")
});

// Get title from HTML
pub fn get_title(text: &str) -> String {
    for titles in Document::from(text).find(Name("title")) {
        if !titles.text().is_empty() {
            return titles.text().trim().to_string();
        }
        if let Some(title) = titles.attr("_html") {
            return title.trim().to_string();
        }
    }
    for titles in Document::from(text).find(Name("meta")) {
        if titles.attr("property") == Some("title") {
            return titles
                .attr("content")
                .unwrap_or_default()
                .trim()
                .to_string();
        }
    }
    if let Some(m) = RE_TITLE.captures(text) {
        return m
            .name("title")
            .map_or("", |m| m.as_str())
            .trim()
            .to_string();
    }
    String::new()
}

// Get favicon from HTML
fn get_favicon_link(text: &str, base_url: &Url) -> HashSet<Url> {
    let mut icon_links = HashSet::new();
    for links in Document::from(text).find(Name("link")) {
        if let (Some(rel), Some(href)) = (links.attr("rel"), links.attr("href")) {
            if RE_COMPILE_BY_SIZE.is_match(href) {
                continue;
            }
            if ["icon", "shortcut icon"].contains(&rel) {
                if href.starts_with("http://") || href.starts_with("https://") {
                    let favicon_url = Url::parse(href).unwrap_or_else(|_| base_url.clone());
                    icon_links.insert(favicon_url);
                } else {
                    let favicon_url = base_url.join(href).unwrap_or_else(|_| base_url.clone());
                    icon_links.insert(favicon_url);
                }
            }
        }
    }
    icon_links
}

// Determine whether it is a picture
fn is_image(headers: &header::HeaderMap, body: &[u8]) -> bool {
    let ct = headers
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| Mime::from_str(value).ok())
        .map(|value| value.type_() == mime::IMAGE)
        .unwrap_or_default();
    let encode_error = String::from_utf8(body.to_vec()).is_err();
    if encode_error {
        let text = String::from_utf8_lossy(body).to_lowercase();
        let is_html = vec!["html", "head", "script", "div"]
            .into_iter()
            .any(|c| text.contains(c));
        ct || !is_html
    } else {
        ct
    }
}

// Get charset from HTML
fn get_charset_from_html(text: &str) -> String {
    for metas in Document::from(text).find(Name("meta")) {
        if let Some(charset) = metas.attr("charset") {
            return charset.to_lowercase();
        }
    }
    String::from("utf-8")
}

// Get the encoding and try to decode, return the decoded string and whether the decoding is successful
fn get_default_encoding(byte: &[u8], headers: header::HeaderMap) -> String {
    let (html, _, _) = UTF_8.decode(byte);
    let default_encoding = get_charset_from_html(&html);
    let content_type: Option<Mime> = headers
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse().ok());
    let header_encoding = content_type
        .as_ref()
        .and_then(|mime| mime.get_param("charset").map(|charset| charset.as_str()))
        .unwrap_or(&default_encoding);
    for encoding_name in &[header_encoding, &default_encoding] {
        let encoding = Encoding::for_label(encoding_name.as_bytes()).unwrap_or(UTF_8);
        let (text, _, is_errors) = encoding.decode(byte);
        if !is_errors {
            return text.to_string();
        }
    }
    if let Ok(text) = String::from_utf8(byte.to_vec()) {
        return text;
    }
    return String::from_utf8_lossy(byte).to_string();
}

// Update status
#[derive(Serialize, Deserialize, Clone, Debug)]
enum Status {
    Pending,
    Done,
    Error,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Pending => write!(f, "Pending"),
            Status::Done => write!(f, "Done"),
            Status::Error => write!(f, "Error"),
        }
    }
}

impl Status {
    fn to_color(&self) -> Color {
        match self {
            Status::Pending => Color::Blue,
            Status::Done => Color::Green,
            Status::Error => Color::Red,
        }
    }
}

// Fill in with xml data to supplement the feed
fn make_page(item: &feed_rs::model::Entry, page_id: PageId) -> HashMap<String, PropertyValue> {
    let item = item.clone();
    let mut links = item.id;
    let published_time = match item.published {
        Some(date) => date,
        None => Utc::now(),
    };
    let updated_time = match item.updated {
        Some(date) => date,
        None => Utc::now(),
    };
    let title = match item.title {
        Some(title) => title.content,
        None => String::new(),
    };
    let summary = match item.summary {
        Some(summary) => summary.content,
        None => String::new(),
    };
    if let Some(l) = item.links.into_iter().next() {
        links = l.href;
    }
    let mut summary_char = String::new();
    for (i, c) in summary.chars().enumerate() {
        summary_char.push(c);
        if i > 100 {
            break;
        }
    }
    let mut page_properties = HashMap::new();
    page_properties.insert(
        "Title".to_string(),
        PropertyValue::Title {
            id: Default::default(),
            title: vec![RichText::Text {
                rich_text: RichTextCommon {
                    plain_text: "".to_string(),
                    href: None,
                    annotations: None,
                },
                text: Text {
                    content: title,
                    link: None,
                },
            }],
        },
    );
    page_properties.insert(
        "Description".to_string(),
        PropertyValue::Text {
            id: Default::default(),
            rich_text: vec![RichText::Text {
                rich_text: RichTextCommon {
                    plain_text: "".to_string(),
                    href: None,
                    annotations: None,
                },
                text: Text {
                    content: summary_char,
                    link: None,
                },
            }],
        },
    );
    page_properties.insert(
        "Link".to_string(),
        PropertyValue::Url {
            id: Default::default(),
            url: Some(links),
        },
    );
    page_properties.insert(
        "Published At".to_string(),
        PropertyValue::Date {
            id: Default::default(),
            date: Some(DateValue {
                start: DateOrDateTime::DateTime(published_time),
                end: None,
                time_zone: None,
            }),
        },
    );
    page_properties.insert(
        "Last Update".to_string(),
        PropertyValue::Date {
            id: Default::default(),
            date: Some(DateValue {
                start: DateOrDateTime::DateTime(updated_time),
                end: None,
                time_zone: None,
            }),
        },
    );
    page_properties.insert(
        "ForeignKey".to_string(),
        PropertyValue::Relation {
            id: Default::default(),
            relation: Some(vec![RelationValue { id: page_id }]),
        },
    );
    page_properties
}

// Database field of the feed
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SourcePage {
    id: PageId,
    icon: Option<FileOrEmojiObject>,
    title: String,
    link: Option<String>,
    last_update_time: Option<DateTime<Utc>>,
    proxy: bool,
    status: Status,
    log: Option<String>,
}

impl Display for SourcePage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, " [Title:{}] ", self.title).unwrap_or_default();
        write!(f, " [Link:{:?}] ", self.link).unwrap_or_default();
        write!(f, " [Status:{}] ", self.status)
    }
}

impl SourcePage {
    // Convert page to SourcePage
    pub fn from_page(page: &Page) -> Self {
        let properties = page.properties.clone();
        let mut links = None;
        let mut last_edited_time = None;
        let mut is_proxy = false;
        if let Some(PropertyValue::Url { url, .. }) = properties.properties.get("Link") {
            links = url.clone();
        }
        if let Some(PropertyValue::Date { date, .. }) = properties.properties.get("Last Update") {
            if let Some(lt) = date.clone() {
                if let DateOrDateTime::DateTime(last_time) = lt.start {
                    last_edited_time = Some(last_time)
                }
            }
        }
        if let Some(PropertyValue::Checkbox { checkbox, .. }) = properties.properties.get("Proxy") {
            is_proxy = *checkbox;
        }
        SourcePage {
            id: page.id.clone(),
            icon: page.icon.clone(),
            title: page.get_title(),
            link: links,
            last_update_time: last_edited_time,
            proxy: is_proxy,
            status: Status::Pending,
            log: None,
        }
    }
    // Return the parsed subscription list
    async fn get_feed_entries(&self, link: Url) -> Result<Vec<feed_rs::model::Entry>> {
        let content = NOTION_FEED
            .client(self.proxy)
            .get(link)
            .send()
            .await?
            .bytes()
            .await?;
        let channels = feed_rs::parser::parse(&content[..])?;
        Ok(channels.entries)
    }
    pub async fn get_feed(mut self) -> Result<SourcePage> {
        match Url::parse(&self.link.clone().unwrap_or_default()) {
            Ok(link) => {
                // Update icon and setting status to pending
                self.update_icon().await.unwrap_or_default();
                let titles = self.get_page_from_database().await.unwrap_or_default();
                match self.get_feed_entries(link).await {
                    Ok(entries) => {
                        for item in entries {
                            // Skip updating if it is an outdated article
                            if let Some(last_time) = self.last_update_time {
                                if let Some(published_time) = item.published {
                                    if last_time > published_time {
                                        continue;
                                    }
                                }
                                if let Some(updated_time) = item.updated {
                                    if last_time > updated_time {
                                        continue;
                                    }
                                }
                            }
                            // Determine whether the title already exists
                            if let Some(t) = item.title.clone() {
                                if titles.contains(&t.content) {
                                    continue;
                                }
                            }
                            let page_properties = make_page(&item, self.id.clone());
                            let page = notion_sdk::pages::CreatePage {
                                icon: None,
                                parent: Parent::Database {
                                    database_id: NOTION_FEED.archive_id.clone(),
                                },
                                properties: Properties {
                                    properties: page_properties,
                                },
                                children: vec![],
                            };
                            let _e = NOTION_FEED.notion.pages_create(page).await;
                        }
                    }
                    Err(e) => {
                        self.status = Status::Error;
                        self.log = Some(e.to_string());
                        self.update_source_page(None).await;
                        return Err(anyhow!(e));
                    }
                }
            }
            Err(e) => {
                self.status = Status::Error;
                self.log = Some(e.to_string());
                self.update_source_page(None).await;
                return Err(anyhow!(e));
            }
        }
        let now = DateValue {
            start: DateOrDateTime::DateTime(Utc::now()),
            end: None,
            time_zone: None,
        };
        self.status = Status::Done;
        self.log = None;
        self.update_source_page(Some(now.clone())).await;
        Ok(self)
    }
    async fn update_source_page(&self, last_time: Option<DateValue>) {
        let mut page_properties = HashMap::new();
        // Do not modify
        if let Some(last_time) = last_time {
            page_properties.insert(
                "Last Update".to_string(),
                PropertyValue::Date {
                    id: Default::default(),
                    date: Some(last_time),
                },
            );
        }
        page_properties.insert(
            "Log".to_string(),
            PropertyValue::Text {
                id: Default::default(),
                rich_text: vec![RichText::Text {
                    rich_text: RichTextCommon {
                        plain_text: "".to_string(),
                        href: None,
                        annotations: None,
                    },
                    text: Text {
                        content: self.log.clone().unwrap_or_default(),
                        link: None,
                    },
                }],
            },
        );
        page_properties.insert(
            "Status".to_string(),
            PropertyValue::Status {
                id: Default::default(),
                status: Some(SelectedValue {
                    id: None,
                    name: Some(self.status.to_string()),
                    color: self.status.to_color(),
                }),
            },
        );
        page_properties.insert(
            "Title".to_string(),
            PropertyValue::Title {
                id: Default::default(),
                title: vec![RichText::Text {
                    rich_text: RichTextCommon {
                        plain_text: "".to_string(),
                        href: None,
                        annotations: None,
                    },
                    text: Text {
                        content: self.title.clone(),
                        link: None,
                    },
                }],
            },
        );
        if let Err(re) = NOTION_FEED
            .notion
            .pages_update(
                self.id.clone(),
                UpdatePage {
                    icon: self.icon.clone(),
                    properties: Properties {
                        properties: page_properties,
                    },
                    archived: false,
                },
            )
            .await
        {
            println!("UpdatePage failed: {:?}", re);
        }
    }
    // Extract the link of favicon from the HTML tag
    async fn find_favicon_tag(&self, base_url: &Url, text: &str) -> Result<String> {
        let icon_sets = get_favicon_link(text, base_url);
        for link in icon_sets {
            let response = NOTION_FEED
                .client(self.proxy)
                .get(link.clone())
                .send()
                .await?;
            let headers = response.headers().clone();
            let status_code = response.status().as_u16();
            let content = response.bytes().await?;
            if status_code == 200 && is_image(&headers, &content) {
                return Ok(link.to_string());
            };
        }
        Err(anyhow!("find_favicon_tag"))
    }
    // If the icon is empty, update the icon
    async fn update_icon(&mut self) -> Result<()> {
        let mut text = String::new();
        if self.icon.is_none() {
            if let Ok(u) = Url::parse(&self.link.clone().unwrap_or_default()) {
                if let Ok(base_url) = u.join("/") {
                    let response = NOTION_FEED
                        .client(self.proxy)
                        .get(base_url.clone())
                        .send()
                        .await?;
                    let headers = response.headers().clone();
                    let text_byte = response.bytes().await?;
                    text = get_default_encoding(&text_byte, headers);
                    let icon_url = self.find_favicon_tag(&base_url, &text).await?;
                    self.icon = Some(FileOrEmojiObject::External {
                        external: ExternalFileObject { url: icon_url },
                    });
                }
            }
        }
        // If the title is empty, update the title
        if self.title.is_empty() && !text.is_empty() {
            self.title = get_title(&text);
        }
        self.status = Status::Pending;
        self.log = None;
        self.update_source_page(None).await;
        Ok(())
    }
    // Get the title associated with the current feed
    async fn get_page_from_database(&self) -> Result<HashSet<String>> {
        let mut titles = HashSet::new();
        let query = DatabaseQuery {
            sorts: None,
            filter: Some(FilterCondition::Property {
                property: "ForeignKey".to_string(),
                condition: PropertyCondition::Relation(RelationCondition::Contains(
                    self.id.clone(),
                )),
            }),
            paging: None,
        };
        let dbs = NOTION_FEED
            .notion
            .databases_query(NOTION_FEED.archive_id.clone(), Some(query))
            .await?;
        if let Object::List { list } = dbs {
            for page in list.results {
                if let Object::Page { page } = page {
                    let tit = page.get_title();
                    titles.insert(tit);
                }
            }
        }
        Ok(titles)
    }
}

// Database field of the feed
#[derive(Clone, Debug)]
pub struct ArchivePage {
    id: PageId,
    title: String,
    link: Option<String>,
    read: bool,
    deleted: bool,
}

impl Display for ArchivePage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, " [Title:{}] ", self.title).unwrap_or_default();
        write!(f, " [Link:{:?}] ", self.link).unwrap_or_default();
        write!(f, " [Read:{}] ", self.read).unwrap_or_default();
        write!(f, " [Deleted:{}] ", self.deleted)
    }
}

impl ArchivePage {
    // Convert page to ArchivePage
    pub fn from_page(page: &Page) -> Self {
        let properties = page.properties.clone();
        let mut links = None;
        let mut is_read = false;
        let mut is_deleted = false;
        if let Some(PropertyValue::Url { url, .. }) = properties.properties.get("Link") {
            links = url.clone();
        }
        if let Some(PropertyValue::Checkbox { checkbox, .. }) = properties.properties.get("Read") {
            is_read = *checkbox;
        }
        if let Some(PropertyValue::Checkbox { checkbox, .. }) = properties.properties.get("Deleted")
        {
            is_deleted = *checkbox;
        }
        ArchivePage {
            id: page.id.clone(),
            title: page.get_title(),
            link: links,
            read: is_read,
            deleted: is_deleted,
        }
    }
    pub async fn delete_me(self) {
        match NOTION_FEED.notion.pages_delete(self.id.clone()).await {
            Ok(_) => {
                println!("Deleted succeeded: {}", self.title.clone());
            }
            Err(err) => {
                println!("Deleted failed: {:?}", err)
            }
        }
    }
}

async fn download_file_from_github(update_url: &str, filename: &str) {
    if let Ok(response) = NOTION_FEED.proxy_client.get(update_url).send().await {
        let mut file = File::create(filename).unwrap();
        let mut content = Cursor::new(response.bytes().await.unwrap_or_default());
        std::io::copy(&mut content, &mut file).unwrap_or_default();
    }
}

pub async fn update_self() {
    // https://doc.rust-lang.org/reference/conditional-compilation.html
    let mut base_url =
        String::from("https://github.com/cn-kali-team/notion-rss/releases/download/default/");
    let mut download_name = "notion-rss_amd64";
    if cfg!(target_os = "windows") {
        download_name = "notion-rss.exe";
    } else if cfg!(target_os = "linux") {
        download_name = "notion-rss-amd64";
    } else if cfg!(target_os = "macos") && cfg!(target_arch = "x86_64") {
        download_name = "notion-rss-darwin";
    } else if cfg!(target_os = "macos") && cfg!(target_arch = "aarch64") {
        download_name = "notion-rss-aarch64-darwin";
    };
    base_url.push_str(download_name);
    let save_filename = "latest_".to_owned() + download_name;
    download_file_from_github(&base_url, &save_filename).await;
    println!(
        "Please rename the file {} to {}",
        save_filename, download_name
    );
}

pub fn read_file_to_feed(file_url: &str) -> HashSet<String> {
    if let Ok(lines) = read_lines(file_url) {
        let target_list: Vec<String> = lines.filter_map(Result::ok).collect();
        return HashSet::from_iter(target_list);
    } else {
        if let Ok(u) = Url::parse(file_url) {
            return HashSet::from_iter(vec![u.to_string()]);
        }
    }
    HashSet::from_iter([])
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
