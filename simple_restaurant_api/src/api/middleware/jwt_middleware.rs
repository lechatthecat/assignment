use actix_web::{body::EitherBody, dev};
use actix_service::Service;
use actix_web::{
    dev::{ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures::future::{ok, Ready, LocalBoxFuture};

use crate::api::jwt::jwt;

pub struct JwtMiddleware;

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = JwtMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtMiddlewareService { service })
    }
}

pub struct JwtMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let is_logged_in = match jwt::verify(&request) {
            Ok(_user_info) => true,
            Err(_) => false
        };

        if !is_logged_in && !(request.path() == "/api/auth/login" || request.path() == "/api/auth/current_user") {
            let (request, _pl) = request.into_parts();

            let response = HttpResponse::Unauthorized()
                .finish()
                // constructed responses map to "right" body
                .map_into_right_body();

            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
        }

        let res = self.service.call(request);

        Box::pin(async move {
            // forwarded responses map to "left" body
            res.await.map(ServiceResponse::map_into_left_body)
        })
    }
}
