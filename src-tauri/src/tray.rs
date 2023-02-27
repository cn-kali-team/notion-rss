use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

pub struct MyTray {}
impl MyTray {
    pub fn tray_menu() -> SystemTrayMenu {
        SystemTrayMenu::new()
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(CustomMenuItem::new("open_window", "Dashboard"))
            .add_item(CustomMenuItem::new("quit", "Quit").accelerator("CmdOrControl+Q"))
    }

    pub fn on_system_tray_event(app_handle: &AppHandle, event: SystemTrayEvent) {
        if let SystemTrayEvent::MenuItemClick { id, .. } = event {
            match id.as_str() {
                "quit" => {
                    app_handle.exit(0);
                    std::process::exit(0);
                }
                "open_window" => create_window(app_handle),
                _ => {}
            }
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
        use window_shadows::set_shadow;

        match builder
            .decorations(false)
            .transparent(true)
            .inner_size(800.0, 636.0)
            .visible(false)
            .build()
        {
            Ok(_) => {
                let app_handle = app_handle.clone();

                if let Some(window) = app_handle.get_window("main") {
                    let _ = set_shadow(&window, true);
                }

                tauri::async_runtime::spawn(async move {
                    sleep(Duration::from_secs(1)).await;

                    if let Some(window) = app_handle.get_window("main") {
                        let _ = window.show();
                        let _ = window.unminimize();
                        let _ = window.set_focus();
                    }
                });
            }
            Err(err) => log::error!(target: "app", "{err}"),
        }
    }

    #[cfg(target_os = "macos")]
    crate::log_err!(builder
        .decorations(true)
        .inner_size(800.0, 642.0)
        .hidden_title(true)
        .title_bar_style(tauri::TitleBarStyle::Overlay)
        .build());

    #[cfg(target_os = "linux")]
    let _ = builder
        .decorations(true)
        .transparent(false)
        .inner_size(800.0, 642.0)
        .build();
}
