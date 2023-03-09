#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use anyhow::Result;
use notion_rss::cli::NotionConfig;
use notion_rss::rss::{add_subscribe, deleted, update};
use notion_rss::tray::MyTray;
use notion_rss::ui::resolve_setup;
use notion_rss::read_file_to_feed;

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

#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", BANNER);
    let config = NotionConfig::default();
    // add subscribe from file
    if let Some(p) = config.file {
        for f in read_file_to_feed(&p) {
            println!("{}", f);
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
    if !config.cli {
        let builder = tauri::Builder::default()
            .system_tray(tauri::SystemTray::new().with_menu(MyTray::tray_menu()))
            .setup(|app| {
                resolve_setup(app);
                Ok(())
            })
            .on_system_tray_event(MyTray::on_system_tray_event)
            .invoke_handler(tauri::generate_handler![
                notion_rss::ui::save_config,
                notion_rss::ui::init_config,
                notion_rss::ui::init_user,
                notion_rss::ui::update_once,
                notion_rss::ui::run_api_server,
                notion_rss::ui::add_feed
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
    } else {
        update(None).await;
    }
    Ok(())
}
