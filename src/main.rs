use axum::{routing::get, Router, response::Html};

async fn home() -> Html<&'static str> {
    Html(r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Rust + Nix Demo</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
            max-width: 800px;
            margin: 80px auto;
            padding: 0 20px;
            line-height: 1.6;
            color: #333;
        }
        h1 { color: #2c3e50; }
        .badge {
            display: inline-block;
            padding: 4px 12px;
            margin: 4px;
            background: #e3f2fd;
            border-radius: 4px;
            font-size: 14px;
        }
        .tech-stack { margin: 30px 0; }
        code {
            background: #f5f5f5;
            padding: 2px 6px;
            border-radius: 3px;
            font-family: 'Courier New', monospace;
        }
    </style>
</head>
<body>
    <h1>🦀 Rust + Nix Demo</h1>
    <p>This web service demonstrates reproducible builds and deployment using Nix flakes.</p>
    
    <div class="tech-stack">
        <h2>Tech Stack</h2>
        <span class="badge">Rust 1.91</span>
        <span class="badge">Axum Web Framework</span>
        <span class="badge">Nix Flakes</span>
        <span class="badge">AWS EC2</span>
    </div>
    
    <h2>What This Demonstrates</h2>
    <ul>
        <li>Reproducible builds with Nix flakes</li>
        <li>Same binary builds locally and on EC2</li>
        <li>Zero-dependency deployment</li>
        <li>Fast iteration with <code>nix develop</code></li>
    </ul>
    
    <h2>Deployment</h2>
    <p>Built on openSUSE Tumbleweed, deployed to Ubuntu 22.04 on AWS EC2 using <code>nix copy</code>.</p>
    
    <hr style="margin: 40px 0; border: none; border-top: 1px solid #ddd;">
    <p style="color: #666; font-size: 14px;">
        Source: <a href="https://codeberg.org/querret/rust-nix-demo">codeberg.org/querret/rust-nix-demo</a>
    </p>
</body>
</html>
    "#)
}

async fn health() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .route("/health", get(health));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    println!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
