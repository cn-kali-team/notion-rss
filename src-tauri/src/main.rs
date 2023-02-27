#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use anyhow::Result;
use notion_rss::api::run_server;
use notion_rss::cli::NotionConfig;
use notion_rss::rss::{add_subscribe, deleted, update};
use notion_rss::tray::MyTray;
use notion_rss::{read_file_to_feed, update_self, NOTION_FEED};
use notion_sdk::pagination::Object;

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
fn init_config() -> NotionConfig {
    NotionConfig::default()
}
#[tauri::command]
async fn init_user() -> Option<notion_sdk::user::User> {
    if let Ok(Object::User { user }) = NOTION_FEED.notion.users_me().await {
        return Some(user);
    }
    None
}
#[tauri::command]
fn save_config(config: NotionConfig) -> String {
    config.save()
}

#[tauri::command]
async fn update_once(window: tauri::Window) {
    update(Some(window.clone())).await;
}

#[tauri::command]
async fn run_api_server(window: tauri::Window) {
    run_server(Some(window));
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", BANNER);
    let config = NotionConfig::default();
    if !config.cli {
        let builder = tauri::Builder::default()
            .system_tray(tauri::SystemTray::new().with_menu(MyTray::tray_menu()))
            .on_system_tray_event(MyTray::on_system_tray_event)
            .invoke_handler(tauri::generate_handler![
                save_config,
                init_config,
                init_user,
                update_once,
                run_api_server
            ]);
        let app = builder
            .build(tauri::generate_context!())
            .expect("error while running tauri application");
        app.run(|app_handle, e| match e {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            tauri::RunEvent::Exit => {
                app_handle.exit(0);
            }
            #[cfg(target_os = "macos")]
            tauri::RunEvent::WindowEvent { label, event, .. } => {
                use tauri::Manager;
                if label == "main" {
                    match event {
                        tauri::WindowEvent::CloseRequested { api, .. } => {
                            api.prevent_close();
                            app_handle.get_window("main").map(|win| {
                                let _ = win.hide();
                            });
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        });
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
        update(None).await;
        std::process::exit(0);
    }
    if config.deleted {
        deleted().await;
        std::process::exit(0);
    }
    Ok(())
}
