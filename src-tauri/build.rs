// only build for windows
#[cfg(target_os = "windows")]
fn main() {
    #[cfg(not(feature = "cli"))]
    tauri_build::build();
    #[cfg(feature = "cli")]
    {
        use winres;
        use std::io::Write;
        // only build the resource for release builds
        // as calling rc.exe might be slow
        if std::env::var("PROFILE").unwrap() == "release" {
            let mut res = winres::WindowsResource::new();
            res.set_icon("icons\\icon.ico");
            match res.compile() {
                Err(e) => {
                    write!(std::io::stderr(), "{}", e).unwrap();
                    std::process::exit(1);
                }
                Ok(_) => {}
            }
        }
    }
}

// nothing to do for other operating systems
#[cfg(not(target_os = "windows"))]
fn main() {
    #[cfg(not(feature = "cli"))]
    tauri_build::build()
}
