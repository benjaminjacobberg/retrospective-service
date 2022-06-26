mod auth;

#[macro_use]
extern crate dotenv_codegen;

use actix_session::storage::CookieSessionStore;
use actix_session::{Session, SessionMiddleware};
use actix_web::cookie::Key;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

use actix_web::web::Data;

use crate::auth::{authorization_code_grant, login, logout, OAuth2Client, SessionKey, User};

#[get("/")]
async fn index(session: Session) -> impl Responder {
    let user_option: Option<User> = session.get(SessionKey::User.as_ref()).ok().flatten();
    return if user_option.is_some() {
        let user = user_option.unwrap();
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::Ok().finish()
    };
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(OAuth2Client::new().clone()))
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                Key::from(&[0; 64]),
            ))
            .service(login)
            .service(logout)
            .service(authorization_code_grant)
            .service(index)
    })
    .bind(dotenv!("SOCKET_ADDRESS"))?
    .run()
    .await
}
