use crate::rss::update;
use crate::ui::create_window;
use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

pub struct MyTray {}

impl MyTray {
    pub fn tray_menu() -> SystemTrayMenu {
        SystemTrayMenu::new()
            .add_item(CustomMenuItem::new("update", "Update"))
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(CustomMenuItem::new("open_window", "Dashboard").accelerator("CmdOrControl+O"))
            .add_item(CustomMenuItem::new("quit", "Quit").accelerator("CmdOrControl+Q"))
    }

    pub fn on_system_tray_event(app_handle: &AppHandle, event: SystemTrayEvent) {
        if let SystemTrayEvent::MenuItemClick { id, .. } = event {
            match id.as_str() {
                "quit" => {
                    app_handle.exit(0);
                    std::process::exit(0);
                }
                "update" => {
                    let h = app_handle.clone();
                    tauri::async_runtime::spawn(async move {
                        update(h.get_window("main")).await;
                    });
                }
                "open_window" => create_window(app_handle),
                _ => {}
            }
        }
    }
}
