use crate::api::run_server;
use crate::cli::NotionConfig;
use crate::rss::update;
use crate::{CONFIG, NOTION_FEED};
use notion_sdk::pagination::Object;
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
    update(Some(window.clone())).await;
}

#[tauri::command]
pub async fn run_api_server(window: tauri::Window) {
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
    .min_inner_size(1200.0, 550.0);

    #[cfg(target_os = "windows")]
    {
        use std::time::Duration;
        use tokio::time::sleep;

        match builder
            .decorations(false)
            .transparent(true)
            .inner_size(800.0, 636.0)
            .visible(false)
            .build()
        {
            Ok(_) => {
                let app_handle = app_handle.clone();
                tauri::async_runtime::spawn(async move {
                    sleep(Duration::from_secs(1)).await;

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
        .inner_size(800.0, 642.0)
        .hidden_title(true)
        .title_bar_style(tauri::TitleBarStyle::Overlay)
        .build();

    #[cfg(target_os = "linux")]
    let _ = builder
        .decorations(true)
        .transparent(false)
        .inner_size(800.0, 642.0)
        .build();
}
