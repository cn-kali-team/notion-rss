use crate::ui::create_window;
use tauri::{AppHandle, CustomMenuItem, SystemTrayEvent, SystemTrayMenu};

pub struct MyTray {}

impl MyTray {
    pub fn tray_menu() -> SystemTrayMenu {
        SystemTrayMenu::new()
            // .add_item(CustomMenuItem::new("add_feed", "Add Feed"))
            // .add_native_item(SystemTrayMenuItem::Separator)
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
                "open_window" => create_window(app_handle),
                _ => {}
            }
        }
    }
}
