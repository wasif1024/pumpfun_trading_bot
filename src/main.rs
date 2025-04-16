mod grpc;
mod sniper;
mod monitor;
use crate::grpc::start_grpc_server;
use tokio::join;
#[tokio::main]
async fn main() {
    env_logger::init();
    
    let grpc_server = tokio::spawn(async move {
        start_grpc_server().await;
    });

    let token_monitor = tokio::spawn(async move {
        monitor::start_monitor().await;
    });

    let _ = join!(grpc_server, token_monitor);
}
