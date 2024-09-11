use std::error::Error;

use axum::{response::IntoResponse, routing::get, Router};
use git2::{Repository};

#[tokio::main]
async fn main() {
    let routers = Router::new()
        .route("/hello", get(handler_hello));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, routers).await.unwrap();
    clone_repository().await.unwrap();
}

async fn handler_hello() -> impl IntoResponse {
    "Hello, world!"
}


async fn clone_repository() -> Result<(), Box<dyn Error>> {
    let dst = "/c/Users/shirayoru/Desktop/tmp/repos/";
    tokio::fs::create_dir_all(dst).await?;
    let repo_url = "git@github.com:Byron/gitoxide.git";

    let repo = Repository::clone(repo_url, dst)?;
    
    Ok(())
}