use actix_web::{web, HttpRequest, HttpResponse, Responder};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use log::{error, info};
use serde::{Deserialize, Serialize};

use crate::{app_state, user};

pub const AUTH: &str = "/auth";
const SECRET: &str = "jwtsecret";
const AUTHORIZATION: &str = "Authorization";
const BEARER: &str = "Bearer";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Claims {
    sub: String,
    iat: usize,
    exp: usize,
}

pub async fn post(
    user: web::Json<user::UserInfo>,
    data: web::Data<app_state::AppState>,
) -> impl Responder {
    info!(">>> recv: auth {}, {}", user.username, user.password);

    let user_list = data.user_list.lock().unwrap();
    let is_valid = user_list.validator(&user);

    if let Some(id) = is_valid {
        let sub = id.to_string();
        let iat = chrono::Utc::now().timestamp();
        let exp = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::seconds(60))
            .unwrap()
            .timestamp();

        let claims = Claims {
            sub,
            iat: iat as usize,
            exp: exp as usize,
        };

        let token = jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(SECRET.as_ref()),
        )
        .unwrap();

        return HttpResponse::Ok().json(token);
    }

    HttpResponse::Unauthorized().json("Incorrect username or password")
}

pub fn auth(req: HttpRequest) -> Option<i32> {
    if let Some(header) = req.headers().get(AUTHORIZATION) {
        if header.to_str().unwrap().starts_with(BEARER) {
            let token = header.to_str().unwrap().trim_start_matches(BEARER).trim();

            if let Ok(token_data) = jsonwebtoken::decode::<Claims>(
                token,
                &DecodingKey::from_secret(SECRET.as_ref()),
                &Validation::default(),
            ) {
                return Some(token_data.claims.sub.parse::<i32>().unwrap());
            }
        }
    }

    error!(">>> auth failed");

    None
}
