use mongodb::{Client, options::ClientOptions};
use mongodb::bson::{doc};
use serde::{Deserialize, Serialize};
use futures::stream::TryStreamExt;


#[tokio::main]
async fn main() {
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();

    // Manually set an option.
    client_options.app_name = Some("My App".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options).unwrap();

    // List the names of the databases in that deployment.
    for db_name in client.list_database_names(None, None).await.unwrap() {
        println!("{}", db_name);
    }

    let db = client.database("tagdb");
    let collection = db.collection::<Doc>("docs");
    // let filter = doc! { "path": "path/to/doc".to_owned() };
    let mut cursor = collection.find(None, None).await.unwrap();

    // Iterate over the results of the cursor.
    while let Some(my_doc) = cursor.try_next().await.unwrap() {
        println!("title: {}", my_doc.path.unwrap());
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Doc {
    path: Option<String>,
    inode: f32,
    tags: Vec<String>
}
