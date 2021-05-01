#[macro_use]
extern crate serde_derive;
mod claims;
pub mod error;
pub mod middleware;
mod tokenizer;

use crate::claims::Claims;
use crate::error::*;
use actix_web::HttpRequest;
use std::env;
use uuid;

pub struct HelixAuth {}
impl HelixAuth {
    pub fn is_auth_token_valid(token: &str) -> HelixAuthResult<()> {
        match HelixAuth::get_token_data(token).is_ok() {
            true => Ok(()),
            false => Err(HelixAuthError::InvalidToken),
        }
    }

    pub fn get_claimer(req: &HttpRequest) -> Option<Claims> {
        match req.headers().get("Authorization") {
            Some(value) => match HelixAuth::get_token_data(&value.to_str().unwrap().to_string()) {
                Ok(data) => Some(data),
                Err(_) => None,
            },
            None => None,
        }
    }

    pub fn generate_tokens(
        user: &str,
        user_uuid: &uuid::Uuid,
        person_uuid: &uuid::Uuid,
    ) -> Result<(String, String), String> {
        let api_auth_key = env::var("HELIX_API_AUTH_KEY").expect("API_AUTH_KEY not found.");

        let tokenizer_access = tokenizer::Tokenizer::new(api_auth_key.to_string());
        let result_access = tokenizer_access
            .claims(claims::get_access_token_claims(
                &user,
                &user_uuid,
                &person_uuid,
            ))
            .generate();

        let tokenizer_refresh = tokenizer::Tokenizer::new(api_auth_key.to_string());
        let result_refresh = tokenizer_refresh
            .claims(claims::get_refresh_token_claims(
                &user,
                &user_uuid,
                &person_uuid,
            ))
            .generate();

        match result_access {
            Ok(a) => match result_refresh {
                Ok(r) => Ok((a, r)),
                Err(_) => Err("Oops, an error occured.".to_owned()),
            },
            Err(_) => (Err("Oops, an error occured.".to_owned())),
        }
    }

    fn get_token_data(token: &str) -> Result<Claims, String> {
        let api_auth_key = env::var("HELIX_API_AUTH_KEY").expect("API_AUTH_KEY not found.");
        let v: Vec<&str> = token.split(' ').collect();
        let tokenizer = tokenizer::Tokenizer::new(api_auth_key.to_string());
        tokenizer
            .validation(claims::get_access_token_validation())
            .validate(&v[1].to_string())
    }

    pub fn refresh_tokens(token: &str) -> Result<(String, String), String> {
        let api_auth_key = env::var("HELIX_API_AUTH_KEY").expect("API_AUTH_KEY not found.");

        let tokenizer = tokenizer::Tokenizer::new(api_auth_key.to_string());
        let result = tokenizer
            .validation(claims::get_refresh_token_validation())
            .validate(token);

        match result {
            Ok(claims) => HelixAuth::generate_tokens(
                &claims.get_user().to_owned(),
                claims.get_user_uuid(),
                claims.get_person_uuid(),
            ),
            Err(_) => Err("Oops, an error occured.".to_owned()),
        }
    }
}
