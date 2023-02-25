#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use anyhow::Result;
use notion_rss::api::run_server;
use notion_rss::rss::{add_subscribe, deleted, update};
use notion_rss::{read_file_to_feed, update_self};
use notion_rss::cli::NotionConfig;

const BANNER: &str = r#"
███╗   ██╗ ██████╗ ████████╗██╗ ██████╗ ███╗   ██╗      ██████╗ ███████╗███████╗
████╗  ██║██╔═══██╗╚══██╔══╝██║██╔═══██╗████╗  ██║      ██╔══██╗██╔════╝██╔════╝
██╔██╗ ██║██║   ██║   ██║   ██║██║   ██║██╔██╗ ██║█████╗██████╔╝███████╗███████╗
██║╚██╗██║██║   ██║   ██║   ██║██║   ██║██║╚██╗██║╚════╝██╔══██╗╚════██║╚════██║
██║ ╚████║╚██████╔╝   ██║   ██║╚██████╔╝██║ ╚████║      ██║  ██║███████║███████║
╚═╝  ╚═══╝ ╚═════╝    ╚═╝   ╚═╝ ╚═════╝ ╚═╝  ╚═══╝      ╚═╝  ╚═╝╚══════╝╚══════╝
Build your own RSS Feeds in Notion.
________________________________________________
:  https://github.com/cn-kali-team/notion-rss  :
:  https://blog.kali-team.cn/donate            :
 -----------------------------------------------
"#;
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", BANNER);
    let config = NotionConfig::default();
    if !config.cli {
        tauri::Builder::default()
            .invoke_handler(tauri::generate_handler![greet])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
    if config.update {
        update_self().await;
        std::process::exit(0);
    }
    // add subscribe from file
    if let Some(p) = config.file {
        for f in read_file_to_feed(&p) {
            match add_subscribe(f).await {
                Ok(t) => {
                    println!("Submitted Successfully: {}.", t);
                }
                Err(e) => {
                    println!("Submitted Failed: {}.", e);
                }
            }
        }
        update().await;
        std::process::exit(0);
    }
    if config.deleted {
        deleted().await;
        std::process::exit(0);
    }
    if let Some(server) = &config.api_server {
        run_server(server);
        std::process::exit(0);
    } else {
        update().await;
    }
    Ok(())
}
