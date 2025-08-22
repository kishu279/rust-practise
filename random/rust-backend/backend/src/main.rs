#![allow(unused)]
use poem::{get, handler, listener::TcpListener, post, web::{Json, Path, Query}, IntoResponse, Route, Server};
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

#[derive(Debug, Deserialize)]
struct User {
    name: String, age: i32
}

#[handler]
fn get_body(Json(data): Json<User>) -> impl IntoResponse {
    let body = &data;

    println!("{:?}", body);

    format!("{:?}", body).into_response()
}


#[tokio::main]
async fn main () -> Result<(), std::io::Error> {
    let app = Route::new().at("/hello/:name", get(hello)).at("/index", index).at("/user", post(get_body));
    Server::new(TcpListener::bind("0.0.0.0:3000")).run(app).await
}