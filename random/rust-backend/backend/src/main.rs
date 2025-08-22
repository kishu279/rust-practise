#![allow(unused)]
use poem::{get, handler, listener::TcpListener, web::{Path, Query}, IntoResponse, Route, Server};
use serde::Deserialize;

#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("hello: {}", name)
}

#[derive(Debug, Deserialize)]
struct Params {
    name: String,
}

#[handler]
fn index(res: Query<Params>) -> impl IntoResponse {
    let name = &res.name;
    println!("{}", name);

    format!("{}", name).into_response()
}

#[tokio::main]
async fn main () -> Result<(), std::io::Error> {
    let app = Route::new().at("/hello/:name", get(hello)).at("/index", index);
    Server::new(TcpListener::bind("0.0.0.0:3000")).run(app).await
}