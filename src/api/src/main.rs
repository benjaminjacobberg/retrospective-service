#[macro_use]
extern crate rocket;

use std::error::Error;

use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct IdToken {
    email: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for IdToken {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<IdToken, Self::Error> {
        let token_opt_result: Option<Result<Vec<u8>, Box<dyn Error>>> = request
            .headers()
            .get_one("x-id-token")
            .map(|encoded_token| base64::decode(encoded_token).map_err(|e| e.into()));
        match token_opt_result {
            Some(Ok(token)) => {
                let id_token: IdToken =
                    serde_json::from_slice(token.as_ref()).expect("id_token expected");
                Outcome::Success(id_token)
            }
            _ => Outcome::Forward(()),
        }
    }
}

#[get("/")]
fn user_info(id_token: IdToken) -> String {
    id_token.email
}

#[get("/", rank = 2)]
fn no_info() -> String {
    String::from("no info")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/v1", routes![user_info, no_info])
}
