// #![allow(unused)]
// use poem::{IntoResponse, Route, Server, get, handler, listener::TcpListener, web::{Path, Query}, http::StatusCode,
// error::ParseQueryError, Response };
// use serde::Deserialize;


// #[handler]
// fn hello(Path(name): Path<String>) -> String {
//     format!("hello : {}", name)
// }

// #[derive(Debug, Deserialize)]
// struct Params {
//     name: String,
// }

// #[handler]
// fn index(Query(params): Query<Params>) -> impl IntoResponse {
//     params.name.into_response()
// }

// #[tokio::main]
// async fn main() -> Result<(), std::io::Error> {
//     let app = Route::new().at("/hello/:name", get(hello)).at("/index", get(index));
//     Server::new(TcpListener::bind("0.0.0.0:3000")).run(app).await
// }


#![allow(unused)]
use poem::{error::ParseQueryError, get, handler, http::StatusCode, listener::TcpListener, web::{Path, Query}, Body, FromRequest, IntoResponse, Request, RequestBody, Response, Route, Server};
use serde::Deserialize;


// get request with path
#[handler]
fn hello(Path(name): Path<String>) -> String {
    println!("Server is working fine");
    format!("Hello : {}", name)
}

#[derive(Debug, Deserialize)]
struct Params {
    name: String
}

// fn index(res: Result<Query<Params>, poem::Error>) -> Result<impl IntoResponse, poem::Error> {
//     match res {
//         Ok(Query(params)) => Ok(params.name.into_response()),
//         Err(err) if err.is::<ParseQueryError>() => Ok(Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(err.to_string())),
//         Err(err) => Err(err),
//     }
// }


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new().at("/hello/:name", get(hello));
    Server::new(TcpListener::bind("0.0.0.0:3000")).run(app).await
}




