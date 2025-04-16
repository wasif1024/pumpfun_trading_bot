use tonic::{transport::Server, Request, Response, Status};
use crate::sniper;
pub mod sniper_proto {
    tonic::include_proto!("sniper_proto");
}

use sniper_proto::sniper_service_server::{SniperService, SniperServiceServer};
use sniper_proto::{SnipeRequest, SnipeResponse};

#[derive(Debug, Default)]
pub struct SniperServiceServiceStruct;

#[tonic::async_trait]
impl SniperService for SniperServiceServiceStruct {
    async fn snipe_token(
        &self,
        request: Request<SnipeRequest>,
    ) -> Result<Response<SnipeResponse>, Status> {
        let token_address = request.into_inner().token_address;
        let success = sniper::snipe(&token_address).await;
        Ok(Response::new(SnipeResponse {
            success,
            message: if success {
                "Sniped successfully".to_string()
            } else {
                "Failed to snipe".to_string()
            },
        }))
    }
}

pub async fn start_grpc_server() {
    let addr = "[::1]:50051".parse().unwrap();
    let service = SniperServiceServiceStruct::default();

    println!("gRPC server running at {}", addr);

    Server::builder()
        .add_service(SniperServiceServer::new(service))
        .serve(addr)
        .await
        .unwrap();
}
