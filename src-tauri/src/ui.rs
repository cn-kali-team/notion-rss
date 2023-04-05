use crate::api::run_server;
use crate::cli::NotionConfig;
use crate::rss::{add_subscribe, update};
use crate::{op_to_url, CONFIG, NOTION_FEED};
use notion_sdk::pagination::Object;
use serde::{Deserialize, Serialize};
use tauri::{App, AppHandle, Manager};
#[tauri::command]
pub fn init_config() -> NotionConfig {
    let c = CONFIG.read().unwrap().clone();
    c
}

#[tauri::command]
pub async fn init_user() -> Option<notion_sdk::user::User> {
    if let Ok(Object::User { user }) = NOTION_FEED.notion.users_me().await {
        return Some(user);
    }
    None
}

#[tauri::command]
pub fn save_config(config: NotionConfig) -> String {
    let mut c = CONFIG.write().unwrap();
    *c = config;
    c.save()
}

#[tauri::command]
pub async fn update_once(window: tauri::Window) {
    #[cfg(not(feature = "cli"))]
    update(Some(window.clone())).await;
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Progress {
    total: usize,
    progress: usize,
}
#[tauri::command]
pub async fn import_feed(content: String, window: tauri::Window) {
    let ul = op_to_url(&content).unwrap_or_default();
    let mut progress = Progress {
        total: ul.len(),
        progress: 0,
    };
    for url in ul {
        match add_subscribe(url).await {
            Ok(t) => {
                window
                    .emit("INFO", format!("Submitted Successfully: {}.", t))
                    .unwrap_or_default();
            }
            Err(e) => {
                window
                    .emit("ERROR", format!("Submitted Failed: {}.", e))
                    .unwrap_or_default();
            }
        }
        progress.progress = progress.progress + 1;
        window
            .emit("PROGRESS", progress.clone())
            .unwrap_or_default();
    }
}

#[tauri::command]
pub async fn add_feed(url: String, window: tauri::Window) {
    match add_subscribe(url).await {
        Ok(t) => {
            window
                .emit("INFO", format!("Submitted Successfully: {}.", t))
                .unwrap_or_default();
        }
        Err(e) => {
            window
                .emit("ERROR", format!("Submitted Failed: {}.", e))
                .unwrap_or_default();
        }
    }
}

#[tauri::command]
pub async fn run_api_server(window: tauri::Window) {
    #[cfg(not(feature = "cli"))]
    run_server(Some(window));
}

pub fn resolve_setup(app: &mut App) {
    app.manage(NotionConfig::default());
    if let Ok(config) = CONFIG.read() {
        if !config.daemon {
            create_window(&app.app_handle());
        }
    }
}

/// create main window
pub fn create_window(app_handle: &AppHandle) {
    if let Some(window) = app_handle.get_window("main") {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }

    let builder = tauri::window::WindowBuilder::new(
        app_handle,
        "main".to_string(),
        tauri::WindowUrl::App("index.html".into()),
    )
    .title("notion-rss")
    .center()
    .fullscreen(false)
    .min_inner_size(1200.0, 550.0)
    .resizable(false);

    #[cfg(target_os = "windows")]
    {
        use window_shadows::set_shadow;
        match builder
            .decorations(true)
            .transparent(false)
            .inner_size(1200.0, 550.0)
            .visible(false)
            .build()
        {
            Ok(_) => {
                let app_handle = app_handle.clone();
                if let Some(window) = app_handle.get_window("main") {
                    let _ = set_shadow(&window, true);
                }
                tauri::async_runtime::spawn(async move {
                    if let Some(window) = app_handle.get_window("main") {
                        let _ = window.show();
                        let _ = window.unminimize();
                        let _ = window.set_focus();
                    }
                });
            }
            Err(err) => println!("{:?}", err),
        }
    }

    #[cfg(target_os = "macos")]
    let _ = builder
        .decorations(true)
        .inner_size(1200.0, 550.0)
        .hidden_title(true)
        .title_bar_style(tauri::TitleBarStyle::Overlay)
        .build();

    #[cfg(target_os = "linux")]
    let _ = builder
        .decorations(true)
        .transparent(false)
        .inner_size(1200.0, 550.0)
        .build();
}
