use tonic::{Request, Response, Status};
use super::wallet_proto::{
    health_service_server::HealthService,
    HealthRequest, HealthResponse,
};

#[derive(Debug, Default)]
pub struct HealthServiceImpl;

#[tonic::async_trait]
impl HealthService for HealthServiceImpl {
    async fn health_check(
        &self,
        _request: Request<HealthRequest>,
    ) -> Result<Response<HealthResponse>, Status> {
        let response = HealthResponse {
            status: "healthy".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            timestamp: chrono::Utc::now().timestamp(),
        };
        
        Ok(Response::new(response))
    }
}