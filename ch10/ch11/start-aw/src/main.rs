use actix_web::{server, App, Error, HttpRequest, Path, Responder, State};
use serde_derive::*;

#[derive(Deserialize)]
struct HelloPath {
    // {name}に対応するフィールド
    name: String,
}

struct MyApp {
    server_name: String,
}

fn hello(req: &HttpRequest) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", to)
}

fn hello_name(to: Path<HelloPath>) -> impl Responder {
    format!("Hello {}", to.name)
}

fn hello_with_state(app: State<MyApp>) -> Result<String, Error> {
    Ok(format!("Hello from {}!", app.server_name))
}

fn run_new() {
    server::new(|| {
        App::new()
            .resource("/", |r| r.f(hello))
            .resource("/{name}", |r| r.with(hello_name))
    })
    .bind("localhost:3000")
    .expect("Can not bind to port 3000")
    .run();
}

fn run_with_state() {
    server::new(|| {
        App::with_state(MyApp {
            server_name: "server with state".into(),
        })
        .resource("/info", |r| r.with(hello_with_state))
    })
    .bind("localhost:3000")
    .expect("Can not bind to port 3000")
    .run();
}

fn main() {
    // run_new();
    run_with_state();
}
