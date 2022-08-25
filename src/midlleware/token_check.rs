use std::future::{ready, Ready};

use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error};
use actix_web::body::EitherBody;
use futures_util::future::LocalBoxFuture;
use crate::cores::biz_code::{from_code, TOKEN_EXPIRE, token_invalid, TOKEN_INVALID};
use crate::models::response::response::Response;
use crate::service::token::validate_token;

pub struct TokenCheck;

impl<S, B> Transform<S, ServiceRequest> for TokenCheck
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = TokenCheckMid<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(TokenCheckMid { service }))
    }
}

pub struct TokenCheckMid<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for TokenCheckMid<S>
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // check token
        let h_map = req.headers();
        let token = h_map.get("token");
        if token.is_none() {
            return Response::<i32>::from_biz_right(from_code(TOKEN_INVALID), req);
        }

        let token = token.unwrap().to_str();
        if token.is_err() {
            return Response::<i32>::from_biz_right(token_invalid(), req);
        }

        let token = token.unwrap();
        if token == "" {
            return Response::<i32>::from_biz_right(from_code(TOKEN_INVALID), req);
        }

        let r = validate_token(token);
        if r.is_err() {
            return Response::<i32>::from_biz_right(token_invalid(), req);
        }

        let is_expire = r.ok().unwrap().is_expire();
        if is_expire {
            return Response::<i32>::from_biz_right(from_code(TOKEN_EXPIRE), req);
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await.map(ServiceResponse::map_into_left_body)?;

            println!("Hi from response");
            Ok(res)
        })
    }
}