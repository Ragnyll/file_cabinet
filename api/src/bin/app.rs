// #![deny(warnings)]
use warp::Filter;

use serde::{Serialize, Deserialize};
use std::net::SocketAddr;
use std::net::IpAddr;

const DEFAULT_IP: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 3000;

#[derive(Serialize, Deserialize, Debug)]
struct Example {
    thing: Option<String>
}

#[tokio::main]
async fn main() {
    let addr = build_ip_addr(DEFAULT_IP, DEFAULT_PORT);

    // Register Routes
    let routes = warp::path!("todos")
                .and(warp::get())
                .and_then(list_todos)
;

    println!("Starting webserver on {}", addr);
    warp::serve(routes).run(addr).await;
    println!("WebServer running on {} shutting down!", addr);
}

fn build_ip_addr(ip: &str, port: u16) -> SocketAddr {
    let ip = ip.parse::<IpAddr>().unwrap();
    SocketAddr::new(ip, port)
}

 async fn list_todos() -> Result<impl warp::Reply, std::convert::Infallible> {
        // Just return a JSON array of todos, applying the limit and offset.
        let obj = Example { thing: Some(String::from("hi")) };
        let todos: Vec<Example> = vec![obj];
            Ok(warp::reply::json(&todos))
 }
