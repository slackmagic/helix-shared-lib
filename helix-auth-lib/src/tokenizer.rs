use crate::Claims;
use jsonwebtoken::{decode, encode, Header, Validation};
use std::result::Result;

pub struct Tokenizer {
    key: String,
    claims: Option<Claims>,
    validation: Option<Validation>,
}

impl Tokenizer {
    pub fn new(key: String) -> Tokenizer {
        Tokenizer {
            key,
            claims: None,
            validation: None,
        }
    }

    pub fn generate(self) -> Result<String, String> {
        match self.claims {
            Some(c) => match encode(&Header::default(), &c, self.key.as_ref()) {
                Ok(s) => Ok(s),
                Err(_) => Err("Error: token encoding failed.".to_owned()),
            },
            None => Err("Error: No claims object set up.".to_owned()),
        }
    }

    pub fn validate(mut self, token: &str) -> Result<Claims, String> {
        match self.validation {
            Some(v) => match decode::<Claims>(&token, self.key.as_ref(), &v) {
                Ok(c) => {
                    self.claims = Some(c.claims);
                    Ok(self.claims.unwrap())
                }

                Err(_) => Err("Error: Invalid token".to_owned()),
            },
            None => Err("Error: No validation object set up.".to_owned()),
        }
    }

    pub fn claims(mut self, claims: Claims) -> Self {
        self.claims = Some(claims);
        self
    }

    pub fn validation(mut self, validation: Validation) -> Self {
        self.validation = Some(validation);
        self
    }
}
