use std::fmt::Debug;

use actix_session::Session;

use actix_web::{get, web, HttpResponse, Responder};

use actix_web::http::header;
use actix_web::web::Data;
use jsonwebtoken::{decode, Algorithm, DecodingKey, TokenData, Validation};
use oauth2::basic::{
    BasicClient, BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse,
    BasicTokenResponse, BasicTokenType,
};
use oauth2::reqwest::{async_http_client, http_client};
use oauth2::{AuthUrl, AuthorizationCode, Client, ClientId, ClientSecret, CsrfToken, RedirectUrl, RevocationUrl, Scope, StandardRevocableToken, TokenResponse, TokenUrl, AccessToken, RefreshToken};
use serde::{Deserialize, Serialize};
use strum_macros::AsRefStr;
use strum_macros::EnumString;

pub(crate) struct OAuth2Client;

impl OAuth2Client {
    pub(crate) fn new() -> BasicClient {
        BasicClient::new(
            ClientId::new(String::from(dotenv!("OAUTH2_CLIENT_ID"))),
            Some(ClientSecret::new(String::from(dotenv!(
                "OAUTH2_CLIENT_SECRET"
            )))),
            AuthUrl::new(String::from(dotenv!("OAUTH2_AUTHORIZATION_URL")))
                .expect("Auth URL expected"),
            Some(
                TokenUrl::new(String::from(dotenv!("OAUTH2_TOKEN_URL")))
                    .expect("Token URL expected"),
            ),
        )
        .set_redirect_uri(
            RedirectUrl::new(String::from(dotenv!("OAUTH2_REDIRECT_URL")))
                .expect("Redirect URL expected"),
        )
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct AuthRequest {
    code: String,
    state: String,
    session_state: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct User {
    email: String,
}

#[derive(AsRefStr, EnumString)]
pub(crate) enum SessionKey {
    User,
    AccessToken,
    RefreshToken,
}

#[get("/auth")]
pub(crate) async fn authorization_code_grant(
    session: Session,
    client: Data<
        Client<
            BasicErrorResponse,
            BasicTokenResponse,
            BasicTokenType,
            BasicTokenIntrospectionResponse,
            StandardRevocableToken,
            BasicRevocationErrorResponse,
        >,
    >,
    params: web::Query<AuthRequest>,
) -> impl Responder {
    let code = AuthorizationCode::new(params.code.clone());
    let token_result = client
        .get_ref()
        .exchange_code(code)
        .request_async(async_http_client)
        .await;

    return match token_result {
        Ok(token) => {
            let access_token: TokenData<User> = decode_token(&token.access_token().secret());
            session
                .insert(SessionKey::User.as_ref(), access_token.claims)
                .expect("User session insert expected");
            session
                .insert(SessionKey::AccessToken.as_ref(), token.access_token())
                .expect("Access Token session insert expected");
            session
                .insert(SessionKey::RefreshToken.as_ref(), token.refresh_token().expect("Refresh Token expected"))
                .expect("Refresh Token session insert expected");
            HttpResponse::Found()
                .append_header((header::LOCATION, "/".to_string()))
                .finish()
        }
        Err(_e) => HttpResponse::InternalServerError().finish(),
    };
}

#[get("/login")]
pub(crate) async fn login(
    client: Data<
        Client<
            BasicErrorResponse,
            BasicTokenResponse,
            BasicTokenType,
            BasicTokenIntrospectionResponse,
            StandardRevocableToken,
            BasicRevocationErrorResponse,
        >,
    >,
) -> impl Responder {
    let (auth_url, _csrf_state) = client
        .get_ref()
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("email".to_string()))
        .url();
    HttpResponse::Found()
        .append_header((header::LOCATION, auth_url.to_string()))
        .finish()
}

#[get("/logout")]
pub(crate) async fn logout(
    session: Session,
    client: Data<
        Client<
            BasicErrorResponse,
            BasicTokenResponse,
            BasicTokenType,
            BasicTokenIntrospectionResponse,
            StandardRevocableToken,
            BasicRevocationErrorResponse,
        >,
    >,
) -> impl Responder {
    let refresh_token_option: Option<RefreshToken> = session.get(SessionKey::RefreshToken.as_ref()).ok().flatten();
    let access_token_option: Option<AccessToken> = session.get(SessionKey::AccessToken.as_ref()).ok().flatten();
    println!("Tokens {:?}, {:?}", refresh_token_option, access_token_option);
    match (refresh_token_option, access_token_option) {
        (Some(rt), Some(at)) => {
            println!("refresh token {}", rt.secret());
            println!("access token {}", at.secret());
            println!("client id {}", dotenv!("OAUTH2_CLIENT_ID"));
        }
        (_, _) => {
            print!(" not found");
        }
    };
    session.remove(SessionKey::User.as_ref());
    session.remove(SessionKey::AccessToken.as_ref());
    session.remove(SessionKey::RefreshToken.as_ref());
    HttpResponse::Found()
        .append_header((header::LOCATION, "/".to_string()))
        .finish()
}

fn decode_token(token: &str) -> TokenData<User> {
    let pem: &str = dotenv!("OAUTH2_KEY_RSA_PEM");
    decode::<User>(
        token,
        &DecodingKey::from_rsa_pem(pem.as_bytes()).unwrap(),
        &Validation::new(Algorithm::RS256),
    )
    .expect("Token data expected")
}
