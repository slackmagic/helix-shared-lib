use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{Error, HttpResponse};
use futures::future::{ok, Either, Ready};

pub struct AuthValidator {
    exception_uri: Vec<String>,
}

impl AuthValidator {
    pub fn new(exception_uri: Vec<String>) -> Self {
        AuthValidator { exception_uri }
    }
}

impl<S, B> Transform<S> for AuthValidator
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthValidatorMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthValidatorMiddleware {
            service,
            exception_uri: self.exception_uri.to_vec(),
        })
    }
}

pub struct AuthValidatorMiddleware<S> {
    service: S,
    exception_uri: Vec<String>,
}

impl<S> AuthValidatorMiddleware<S> {
    fn is_api_call(&self, uri: &str) -> bool {
        uri.starts_with("/api")
    }
    fn is_exception_uri(&self, uri: &str) -> bool {
        let search: String = uri.replace("//", "/");
        self.exception_uri.contains(&search)
    }
    fn is_auth_token_valid(&self, token: &str) -> bool {
        //TODO: need shared library
        true
    }
}

impl<S, B> Service for AuthValidatorMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        // We only need to hook into the `start` for this middleware.

        //Check if the route is excluded.
        let uri = &req.uri().to_string();
        if !self.is_api_call(uri) || self.is_exception_uri(uri) {
            Either::Left(self.service.call(req))
        } else {
            //Valid Authorization header
            match req.headers().get("Authorization") {
                Some(value) => {
                    if self.is_auth_token_valid(&value.to_str().unwrap().to_string()) {
                        Either::Left(self.service.call(req))
                    } else {
                        //Auth NOT OK"
                        Either::Right(ok(
                            req.into_response(HttpResponse::Unauthorized().finish().into_body())
                        ))
                    }
                }
                None => {
                    //NO Auth Token"
                    Either::Right(ok(req.into_response(
                        HttpResponse::ExpectationFailed().finish().into_body(),
                    )))
                }
            }
        }
    }
}
