use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn start() -> Result<(), Box<dyn std::error::Error>> {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World from WS!" }));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3001".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error Starting WS API: {}", err);
    }
}