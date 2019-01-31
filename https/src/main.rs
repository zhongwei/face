extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate rustls;

#[macro_use]
extern crate rust_embed;
extern crate mime_guess;

use std::fs::File;
use std::io::BufReader;
use actix_web::http::Method;
use actix_web::{server, App, Body, HttpRequest, HttpResponse};
use rustls::internal::pemfile::{certs, rsa_private_keys};
use rustls::{NoClientAuth, ServerConfig};
use server::ServerFlags;

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
    if ::std::env::var("RUST_LOG").is_err() {
        ::std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    // load ssl keys
    let mut config = ServerConfig::new(NoClientAuth::new());
    let cert_file = &mut BufReader::new(File::open("cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("key.pem").unwrap());
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = rsa_private_keys(key_file).unwrap();
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

    // actix acceptor
    let acceptor = server::RustlsAcceptor::with_flags(
        config,
        ServerFlags::HTTP1 | ServerFlags::HTTP2,
    );

    if open::that("https://localhost:8000").is_ok() {
      println!("Look at your browser !");
    };

    server::new(|| App::new().route("/", Method::GET, index).route("/{_:.*}", Method::GET, dist))
    .bind_with("0.0.0.0:8000", move || acceptor.clone())
    .unwrap()
    .run();

}