use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[cfg(not(target_os = "windows"))]
extern crate daemonize;

use crate::rss::{add_subscribe, update};
use crate::{NOTION_FEED, NOTION_RSS_PATH};
use actix_web::http::header::ContentType;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
#[cfg(not(target_os = "windows"))]
use daemonize::Daemonize;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
#[cfg(not(target_os = "windows"))]
use std::fs::File;
use std::net::SocketAddr;
use std::str::FromStr;
use std::thread;

fn get_ssl_config() -> Result<SslAcceptorBuilder> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    let key_path = NOTION_RSS_PATH.join("key.pem");
    let cert_path = NOTION_RSS_PATH.join("cert.pem");
    builder.set_private_key_file(key_path, SslFiletype::PEM)?;
    builder.set_certificate_chain_file(cert_path)?;
    Ok(builder)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Query {
    // FeedBin
    subscribe: Option<String>,
    // Nextcloud News
    subscribe_to: Option<String>,
    // Miniflux
    uri: Option<String>,
    // Tiny Tiny RSS
    feed_url: Option<String>,
    //FreshRSS
    url_rss: Option<String>,
    //Inoreader
    add_feed: Option<String>,
}

const HTML: &str = r#"
<!DOCTYPE html>
<html>
<head>
	<title>NOTION-RSS</title>
	<meta charset="UTF-8">
	<script>
		window.onload = function() {
			var count = 5;
			var countdown = setInterval(function() {
				document.getElementById('countdown').innerHTML = count;
				count--;
				if (count < 0) {
					clearInterval(countdown);
					window.location.href="about:blank";
					window.close();
				}
			}, 1000);
		};
	</script>
</head>
<body>
	<div style="display: flex; justify-content: center; align-items: center;">
	    <h1>Kali-Team</h1>
	</div>
	<div style="display: flex; justify-content: center; align-items: center;">
	    <p>The page will automatically close in <span id="countdown">5</span> seconds.</p>
	</div>
</body>
</html>

"#;

async fn subscribe(query: web::Query<Query>) -> impl Responder {
    let mut text: Result<String> = Err(anyhow!("Path error, please check Token parameter."));
    if let Some(u) = &query.subscribe {
        text = add_subscribe(u.to_string()).await;
    } else if let Some(u) = &query.subscribe_to {
        text = add_subscribe(u.to_string()).await;
    } else if let Some(u) = &query.add_feed {
        text = add_subscribe(u.to_string()).await;
    } else if let Some(u) = &query.feed_url {
        text = add_subscribe(u.to_string()).await;
    } else if let Some(u) = &query.uri {
        text = add_subscribe(u.to_string()).await;
    } else if let Some(u) = &query.url_rss {
        text = add_subscribe(u.to_string()).await;
    }
    let mut html = HTML.to_string();
    match text {
        Ok(t) => {
            let msg = format!("Submitted Successfully: {}.", t);
            html = html.replace("Kali-Team", &msg);
            println!("{}", msg);
            tokio::task::spawn(async move {
                #[cfg(feature = "cli")]
                update().await;
                #[cfg(not(feature = "cli"))]
                update(None).await;
            });
        }
        Err(e) => {
            let msg = format!("Submitted Failed: {}.", e);
            html = html.replace("Kali-Team", &msg);
            println!("{}", msg);
        }
    }
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html)
}

async fn not_found() -> impl Responder {
    let mut html = HTML.to_string();
    html = html.replace("Kali-Team", "Path error, please check Token parameter.");
    HttpResponse::NotFound()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[actix_web::main]
async fn api_server(listening_address: SocketAddr, token: String) {
    std::env::set_var("RUST_LOG", "actix_web=info");
    let token_path = token.clone();
    let http_server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .route(&format!("/{}/", token_path), web::get().to(subscribe))
            .route(
                &format!("/{}/public.php", token_path),
                web::get().to(subscribe),
            )
            .route(
                &format!("/{}/bookmarklet", token_path),
                web::get().to(subscribe),
            )
            .route(&format!("/{}/i/", token_path), web::get().to(subscribe))
            .default_service(web::route().to(not_found))
    });
    let mut s = format!("http://{}/{}/", listening_address, token);
    println!("Copy the URL to your RssHub browser plug-in configuration");
    println!("Configure any of the following");
    println!("[Tiny Tiny RSS, Miniflux, FreshRSS, Nextcloud News, InoReader, FeedBin]");
    if let Ok(ssl_config) = get_ssl_config() {
        let https_server = http_server.bind_openssl(listening_address, ssl_config);
        s = s.replace("http://", "https://");
        if let Ok(server) = https_server {
            println!("API service has been started: {}", s);
            server.workers(32).run().await.unwrap_or_default();
        }
    } else {
        let http_server = http_server.bind(listening_address);
        if let Ok(server) = http_server {
            println!("API service has been started: {}", s);
            server.workers(32).run().await.unwrap_or_default();
        }
    }
}

// Start web service
#[cfg(feature = "cli")]
pub fn run_server() {
    let server = match NOTION_FEED.config.api_server.clone() {
        Some(server) => server,
        None => String::new(),
    };
    if NOTION_FEED.config.daemon && NOTION_FEED.config.cli {
        background();
    }
    if let Ok(address) = SocketAddr::from_str(&server) {
        thread::spawn(move || {
            api_server(address, NOTION_FEED.config.token.clone());
        });
    } else {
        println!("Invalid listening address");
    }
}

#[cfg(not(feature = "cli"))]
pub fn run_server(window: Option<tauri::Window>) {
    let server = match NOTION_FEED.config.api_server.clone() {
        Some(server) => server,
        None => String::new(),
    };
    if NOTION_FEED.config.daemon && NOTION_FEED.config.cli {
        background();
    }
    if let Ok(address) = SocketAddr::from_str(&server) {
        thread::spawn(move || {
            api_server(address, NOTION_FEED.config.token.clone());
        });
    } else if let Some(w) = window {
        w.emit("ERROR", "Invalid listening address")
            .unwrap_or_default();
    } else {
        println!("Invalid listening address");
        return;
    }
}

#[cfg(not(target_os = "windows"))]
fn background() {
    let stdout = File::create("/tmp/notion-rss.out").unwrap();
    let stderr = File::create("/tmp/notion-rss.err").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("/tmp/notion-rss.pid") // Every method except `new` and `start`
        .chown_pid_file(false) // is optional, see `Daemonize` documentation
        .working_directory("/tmp") // for default behaviour.
        .user("nobody")
        .group("daemon") // Group name
        .umask(0o777) // Set umask, `0o027` by default.
        .stdout(stdout) // Redirect stdout to `/tmp/notion-rss.out`.
        .stderr(stderr) // Redirect stderr to `/tmp/notion-rss.err`.
        .exit_action(|| println!("Executed before master process exits"))
        .privileged_action(|| "Executed before drop privileges");
    match daemonize.start() {
        Ok(_) => println!("Success, daemonized"),
        Err(e) => eprintln!("Error, {}", e),
    }
}

#[cfg(target_os = "windows")]
fn background() {
    println!("Windows does not support background services");
}
