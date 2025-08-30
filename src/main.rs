mod commandes;
mod cores;
mod handler;
use crate::handler::Handler;
use cores::api::config_api;
#[tokio::main]
async fn main() {
    Handler::new().await;
    config_api().await;
}
