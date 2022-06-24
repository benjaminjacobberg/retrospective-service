#[macro_use]
extern crate dotenv_codegen;

use std::error::Error;
use actix_web::{get, web, App, HttpServer, Responder};
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let client =
        BasicClient::new(
            ClientId::new(String::from(dotenv!("OAUTH2_CLIENT_ID"))),
            Some(ClientSecret::new(String::from(dotenv!("OAUTH2_CLIENT_SECRET")))),
            AuthUrl::new(String::from(dotenv!("OAUTH2_AUTHORIZATION_URL"))).unwrap(),
            Some(TokenUrl::new(String::from(dotenv!("OAUTH2_TOKEN_URL"))).unwrap()),
        ).set_redirect_uri(RedirectUrl::new("https://localhost:8080/".to_string()).unwrap());
    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("user:email".to_string()))
        .url();

    HttpServer::new(|| {
        App::new().service(greet)
    })
        .bind(dotenv!("SOCKET_ADDRESS"))?
        .run()
        .await
}