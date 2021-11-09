// #![deny(warnings)]
use warp::Filter;

use serde::{Serialize, Deserialize};
use std::net::SocketAddr;
use std::net::IpAddr;
use std::str::FromStr;
use mongodb::{Client, options::ClientOptions};
use mongodb::bson::{doc};
use futures::stream::TryStreamExt;
use warp::{
    http::{header, Method},
};

const DEFAULT_IP: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 3000;

const DB_HOST: &str = "mongodb://localhost:27017";

#[derive(Serialize, Deserialize, Debug)]
struct Doc {
    path: Option<String>,
    inode: f32,
    tags: Vec<String>,
}

struct Offset {
    start: u64,
}

impl FromStr for Offset {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Offset {
            start: s.parse::<u64>()?,
        })
    }
}

#[tokio::main]
async fn main() {
    let addr = build_ip_addr(DEFAULT_IP, DEFAULT_PORT);

    let offset_header = warp::header::<Offset>("64");

    // Register Routes
    let routes = warp::path!("docs")
        .and(warp::get())
        .and_then(list_all_doc_tags);

    println!("==== Starting webserver on {}", addr);
    warp::serve(routes).run(addr).await;
    println!("==== WebServer running on {} shutting down!", addr);
}

fn build_ip_addr(ip: &str, port: u16) -> SocketAddr {
    let ip = ip.parse::<IpAddr>().unwrap();
    SocketAddr::new(ip, port)
}

async fn list_all_doc_tags() -> Result<impl warp::Reply, std::convert::Infallible> {
    let client_options = ClientOptions::parse(DB_HOST).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("tagdb");
    let collection = db.collection::<Doc>("docs");
    let mut cursor = collection.find(None, None).await.unwrap();

    let mut docs = vec![];
    // Iterate over the results of the cursor.
    while let Some(my_doc) = cursor.try_next().await.unwrap() {
        docs.push(my_doc);
    }
    Ok(warp::reply::json(&docs))
}
