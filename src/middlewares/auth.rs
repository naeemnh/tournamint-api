use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    Error, HttpMessage,
};
use futures_util::future::{ok, Ready};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;

use crate::models::auth::Claims;

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareService {
            service: Rc::new(service),
        })
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(
        &self,
        ctx: &mut core::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        Box::pin(async move {
            // Skip auth for certain paths
            if req.path().starts_with("/auth") {
                return svc.call(req).await;
            }

            // Extract token from Authorization header
            let token = req
                .headers()
                .get("Authorization")
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.strip_prefix("Bearer "))
                .ok_or_else(|| ErrorUnauthorized("Missing token"))?;

            // Validate JWT
            let secret = std::env::var("JWT_SECRET")
                .map_err(|_| ErrorUnauthorized("Server configuration error"))?;

            let token_data = decode::<Claims>(
                token,
                &DecodingKey::from_secret(secret.as_bytes()),
                &Validation::new(Algorithm::HS256),
            )
            .map_err(|_| ErrorUnauthorized("Invalid token"))?;

            // Attach claims to request
            req.extensions_mut().insert(token_data.claims);

            svc.call(req).await
        })
    }
}
