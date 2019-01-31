extern crate actix;
extern crate actix_web;

#[macro_use]
extern crate rust_embed;
extern crate mime_guess;

use actix_web::http::Method;
use actix_web::{server, App, Body, HttpRequest, HttpResponse};

use mime_guess::guess_mime_type;
use std::borrow::Cow;

#[derive(RustEmbed)]
#[folder = "public/"]
struct Asset;

fn handle_embedded_file(path: &str) -> HttpResponse {
  match Asset::get(path) {
    Some(content) => {
      let body: Body = match content {
        Cow::Borrowed(bytes) => bytes.into(),
        Cow::Owned(bytes) => bytes.into(),
      };
      HttpResponse::Ok().content_type(guess_mime_type(path).as_ref()).body(body)
    }
    None => HttpResponse::NotFound().body("404 Not Found"),
  }
}

fn index(_req: HttpRequest) -> HttpResponse {
  handle_embedded_file("index.html")
}

fn dist(req: HttpRequest) -> HttpResponse {
  let path = &req.path()["/".len()..]; // trim the preceding `/dist/` in path
  handle_embedded_file(path)
}

fn main() {
    if open::that("http://localhost:8000").is_ok() {
      println!("Look at your browser !");
    };

    server::new(|| App::new().route("/", Method::GET, index).route("/{_:.*}", Method::GET, dist))
    .bind("0.0.0.0:8000")
    .unwrap()
    .run();
}