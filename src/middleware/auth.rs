// src/middleware/auth.rs
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};
use std::rc::Rc;

pub struct ApiKeyAuth;

impl<S, B> Transform<S, ServiceRequest> for ApiKeyAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = ApiKeyAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ApiKeyAuthMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct ApiKeyAuthMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for ApiKeyAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        
        Box::pin(async move {
            // Skip auth for health check endpoint
            if req.path() == "/api/v1/health" {
                return service.call(req).await;
            }

            // Get API key from environment
            let expected_api_key = std::env::var("API_KEY")
                .expect("API_KEY must be set in environment");

            // Check for API key in headers
            let api_key = req.headers()
                .get("x-api-key")
                .and_then(|h| h.to_str().ok());

            match api_key {
                Some(key) if key == expected_api_key => {
                    // Valid API key, proceed with request
                    service.call(req).await
                }
                _ => {
                    // Invalid or missing API key
                    let response = HttpResponse::Unauthorized()
                        .json(serde_json::json!({
                            "error": {
                                "code": 401,
                                "message": "Unauthorized: Invalid or missing API key",
                                "type": "authentication_error"
                            }
                        }));
                    
                    Ok(req.into_response(response))
                }
            }
        })
    }
}