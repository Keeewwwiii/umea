
use std::fs;
use std::io;
use std::path::Path;

pub fn execute(name: &str) -> io::Result<()> {
    let root_path = Path::new("examples").join(name);
    let public_path = root_path.join("public");
    let src_path = public_path.join("src");

    fs::create_dir_all(&src_path)?;

    let version = 1.0; 
    let umea_json = format!(
r#"
{{
    "name": "{}",
    "version": {},
    "permissions": [],
    "root": "public/"
}}
"#,
        name, version
    );

    let html: &str = r#"<!doctype html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <title>My App</title>
        <link rel="stylesheet" href="src/style.css">
    </head>
    <body>
        <h1>Hello from my app!</h1>
    </body>
    </html>
    "#;

        let css: &str = r#"
    *{
        margin: 0;
        padding: 0;
    }
    body{
        height: 100vh;
        width: 100%;
        display: flex;
        align-items: center;
        place-content: center;
        color: white;
        font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
        background-color: #15d170;
    }
    "#;

    fs::write(root_path.join("umea.json"), umea_json)?;
    fs::write(public_path.join("index.html"), html)?;
    fs::write(src_path.join("style.css"), css)?; 

    println!("Created app: {name}");

    Ok(())
}