use axum::{routing::get, Router};

pub async fn run() {
    // Adding an endpoint: http://localhost:4000/ping
    let app = Router::new()
        .route("/ping", get(ping));
    
    println!("Server listening on port 4000"); 
    axum::Server::bind(&"127.0.0.1:4000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}

async fn ping() -> String {
    "Server was pinged".to_owned()
}