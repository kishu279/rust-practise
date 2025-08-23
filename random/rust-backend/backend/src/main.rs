#![allow(unused)]
use poem::{get, handler, listener::TcpListener, post, web::{Json, Path, Query}, IntoResponse, Route, Server};
use serde::Deserialize;
use store::store::Store;

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

#[derive(Debug, Deserialize)]
struct NewUser {
    name: String,
    email: String,
}

#[handler]
fn create_user (Json(data): Json<NewUser>) -> impl IntoResponse {
    let body = &data;
    println!("{:?}", body);

    let store = Store::default().expect("failed to get the store");
    let id = store.create_user(body.name, body.email);

    // Json({id})
    id
}


#[tokio::main]
async fn main () -> Result<(), std::io::Error> {
    let app = Route::new().at("/hello/:name", get(hello)).at("/index", index).at("/user", post(get_body));
    Server::new(TcpListener::bind("0.0.0.0:3000")).run(app).await
}