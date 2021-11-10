// #![deny(warnings)]
use std::net::SocketAddr;
use std::net::IpAddr;
use api::routing::routes;
use mongodb::{Client, options::ClientOptions};

const DEFAULT_IP: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 3000;
const DB_HOST: &str = "mongodb://localhost:27017";

async fn initialize_db_client() -> Client {
    let client_options = ClientOptions::parse(DB_HOST).await.unwrap();
    Client::with_options(client_options).unwrap()
}

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "todos=info");
    let addr = build_ip_addr(DEFAULT_IP, DEFAULT_PORT);

    let client = initialize_db_client().await;

    let rest_api = routes::mount_routes(client);

    println!("==== Starting webserver on {}", addr);
    warp::serve(rest_api).run(addr).await;
    println!("==== WebServer running on {} shutting down!", addr);
}

fn build_ip_addr(ip: &str, port: u16) -> SocketAddr {
    let ip = ip.parse::<IpAddr>().unwrap();
    SocketAddr::new(ip, port)
}
