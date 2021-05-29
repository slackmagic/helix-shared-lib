use chrono::prelude::*;
use chrono::Duration;
use jsonwebtoken::Validation;
use std::env;
use uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub user: String,
    pub user_uuid: uuid::Uuid,
    pub person_uuid: uuid::Uuid,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn get_user(&self) -> &String {
        &self.user
    }

    pub fn get_user_uuid(&self) -> &uuid::Uuid {
        &self.user_uuid
    }

    pub fn get_person_uuid(&self) -> &uuid::Uuid {
        &self.person_uuid
    }
}

pub fn get_access_token_claims(
    user: &str,
    user_uuid: &uuid::Uuid,
    person_uuid: &uuid::Uuid,
) -> Claims {
    let sub = "access-token".to_owned();
    let iss = env::var("API_HOSTNAME").expect("API_HOSTNAME not found.");
    let exp: i64 = env::var("HELIX_ACCESS_TOKEN_MAX_LIFETIME")
        .expect("HELIX_ACCESS_TOKEN_MAX_LIFETIME not found.")
        .parse()
        .unwrap();

    get_token_claims(&iss, &sub, user, user_uuid, person_uuid, exp)
}

pub fn get_refresh_token_claims(
    user: &str,
    user_uuid: &uuid::Uuid,
    person_uuid: &uuid::Uuid,
) -> Claims {
    let sub = "refresh-token".to_owned();
    let iss = env::var("API_HOSTNAME").expect("API_HOSTNAME not found.");
    let exp: i64 = env::var("HELIX_REFRESH_TOKEN_MAX_LIFETIME")
        .expect("HELIX_REFRESH_TOKEN_MAX_LIFETIME not found.")
        .parse()
        .unwrap();

    get_token_claims(&iss, &sub, user, user_uuid, person_uuid, exp)
}

pub fn get_access_token_validation() -> Validation {
    let sub = "access-token".to_owned();
    let iss = env::var("HOSTNAME").expect("HOSTNAME not found.");

    get_token_validation(&iss, &sub)
}

#[allow(dead_code)]
pub fn get_refresh_token_validation() -> Validation {
    let sub = "refresh-token".to_owned();
    let iss = env::var("HOSTNAME").expect("HOSTNAME not found.");

    get_token_validation(&iss, &sub)
}

fn get_token_claims(
    iss: &str,
    sub: &str,
    user: &str,
    user_uuid: &uuid::Uuid,
    person_uuid: &uuid::Uuid,
    exp: i64,
) -> Claims {
    let utc: DateTime<Utc> = Utc::now();
    Claims {
        iss: iss.to_owned(),
        sub: sub.to_owned(),
        user: user.to_owned(),
        user_uuid: *user_uuid,
        person_uuid: *person_uuid,
        exp: (utc + Duration::minutes(exp)).timestamp(),
        iat: utc.timestamp(),
    }
}

fn get_token_validation(iss: &str, sub: &str) -> Validation {
    Validation {
        iss: Some(iss.to_owned()),
        sub: Some(sub.to_owned()),
        ..Validation::default()
    }
}
