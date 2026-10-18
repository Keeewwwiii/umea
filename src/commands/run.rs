
use axum::Router;
use serde_json::Value;

use std::fs;

use std::{
    error::Error,
    io,
    path::PathBuf,
    process::Command,
};
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};

type AppResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

pub async fn execute(path: PathBuf, port: u16, standalone:bool) -> AppResult<()> {
    if !path.is_dir() {
        return Err(format!(
            "App directory does not exist: {}",
            path.display()
        )
        .into());
    }
    let data: String = fs::read_to_string(path.join("umea.json"))?;
    let v: Value = serde_json::from_str(&data)?;
    let root_dir;
    if let Some(root_str) = v["root"].as_str() {
        root_dir = path.join(root_str);
        let index_path = root_dir.join("index.html");

        if !index_path.is_file() {
            return Err(format!(
                "Missing index.html in {}",
                index_path.display()
            )
            .into());
        }

        // index to /
        let app = Router::new()
            .route_service("/", ServeFile::new(index_path))
            .fallback_service(ServeDir::new(root_dir.clone()));

        // server host
        let address = format!("127.0.0.1:{port}");
        let listener = TcpListener::bind(&address).await?;

        let url = format!("http://{address}/");

        println!("Starting umea app: {}", root_dir.display());
        println!("Open your app at: {url}");
        println!("Press Ctrl+C to stop the server.");

        let launch_result = if standalone {
            open_standalone(&url)
        } else {
            open_browser(&url)
        };

        if let Err(error) = launch_result {
            eprintln!("Could not launch the app: {error}");
            eprintln!("Open this URL manually: {url}");
        }
        axum::serve(listener, app).await?;

        
    }
    Ok(())



    
}
fn open_standalone(url: &str) -> io::Result<()> {
    #[cfg(target_os = "windows")]
    {
        let mut roots = Vec::new();

        for variable in [
            "PROGRAMFILES",
            "PROGRAMFILES(X86)",
            "LOCALAPPDATA",
        ] {
            if let Ok(value) = std::env::var(variable) {
                roots.push(PathBuf::from(value));
            }
        }

        let mut candidates: Vec<(PathBuf, bool)> = Vec::new();

        for root in roots {
            candidates.extend([
                (
                    root.join("Google/Chrome/Application/chrome.exe"),
                    true,
                ),
                (
                    root.join("Microsoft/Edge/Application/msedge.exe"),
                    true,
                ),
                (
                    root.join(
                        "BraveSoftware/Brave-Browser/Application/brave.exe",
                    ),
                    true,
                ),
                (
                    root.join("Vivaldi/Application/vivaldi.exe"),
                    true,
                ),
                (
                    root.join("Chromium/Application/chrome.exe"),
                    true,
                )
                
            ]);
        }

        for (browser, app_mode) in candidates {
            if !browser.is_file() {
                continue;
            }

            let mut command = Command::new(browser);

            if app_mode {
                command.arg(format!("--app={url}"));
            } else {
                command.args(["-new-window", url]);
            }

            command.spawn()?;
            return Ok(());
        }
    }

    #[cfg(target_os = "macos")]
    {
        let browsers = [
            ("Google Chrome", true),
            ("Microsoft Edge", true),
            ("Brave Browser", true),
            ("Vivaldi", true),
            ("Chromium", true),
          
        ];

        for (browser, app_mode) in browsers {
            let mut command = Command::new("open");
            command.args(["-a", browser, "--args"]);

            if app_mode {
                command.arg(format!("--app={url}"));
            } else {
                command.args(["-new-window", url]);
            }

            if let Ok(status) = command.status() {
                if status.success() {
                    return Ok(());
                }
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        let browsers = [
            ("google-chrome", true),
            ("google-chrome-stable", true),
            ("microsoft-edge", true),
            ("microsoft-edge-stable", true),
            ("brave-browser", true),
            ("brave-browser-stable", true),
            ("vivaldi", true),
            ("chromium", true),
            ("chromium-browser", true),
            ("firefox", false),
        ];

        for (browser, app_mode) in browsers {
            let mut command = Command::new(browser);

            if app_mode {
                command.arg(format!("--app={url}"));
            } else {
                command.args(["-new-window", url]);
            }

            if command.spawn().is_ok() {
                return Ok(());
            }
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "No supported browser could be launched in standalone mode",
    ))
}
fn open_browser(url: &str) -> io::Result<()> {
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(["/C", "start", "", url])
            .spawn()?;
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open").arg(url).spawn()?;
    }

    #[cfg(all(
        unix,
        not(target_os = "macos")
    ))]
    {
        Command::new("xdg-open").arg(url).spawn()?;
    }

    Ok(())
}
