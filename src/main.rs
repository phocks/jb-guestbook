extern crate dotenv;

#[macro_use]
extern crate dotenv_codegen;

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use couch_rs::document::DocumentCollection;
use couch_rs::types::find::FindQuery;
use dotenv::dotenv;
use serde_json::json;
use serde_json::Value;
use std::env;
use std::error::Error;
use tokio;
use warp::Filter;

const DB_HOST: &str = "http://localhost:5984";
const TEST_DB: &str = "test_db";

fn fetch_data() -> Vec<&'static str> {
    vec!["hello", "world"]
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Just a test
    let a = 3;
    let b = 1 + 2;
    assert_eq!(a, b);

    // Don't panic???
    dotenv().ok();
    dotenv().expect(".env file not found");

    let client = couch_rs::Client::new(DB_HOST, "admin", dotenv!("COUCHDB_PASSWORD"))?;
    let db = client.db(TEST_DB).await?;
    let find_all = FindQuery::find_all();
    let docs = db.find_raw(&find_all).await?;

    println!("{:#?}", docs.get_data());

    // GET /ids returns a `200 OK` with a JSON array of ids:
    // `[1, 3, 7, 13]`
    // let routes = warp::path("ids").map(|| {
    //     let our_ids = vec![1, 3, 7, 13];
    //     warp::reply::json(&our_ids)
    // });

    // Match any request and return hello world!
    let routes = warp::any().map(move || {
        let our_ids = fetch_data();
        // let our_ids = docs.get_data();
        warp::reply::json(&our_ids)
    });

    println!("Attempting to listen on http://127.0.0.1:65000/");

    warp::serve(routes).run(([0, 0, 0, 0], 65000)).await;
    Ok(())
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     // Load the MongoDB connection string from an environment variable:
//     let client_uri = "mongodb://localhost:27017";
//     //   env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
//     // A Client is needed to connect to MongoDB:
//     let client = mongodb::Client::with_uri_str(client_uri.as_ref()).await?;
//     // Print the databases in our MongoDB cluster:
//     println!("Databases:");

//     let databases = client.list_database_names(None, None).await?;

//     for name in databases {
//         println!("- {}", name);
//     }

//     let new_doc = doc! {
//        "title": "Parasite",
//        "year": 2020,
//        "plot": "A poor family, the Kims, con their way into becoming the servants of a rich family, the Parks. But their easy life gets complicated when their deception is threatened with exposure.",
//        "released": Utc.ymd(2020, 2, 7).and_hms(0, 0, 0),
//     };

//     println!("{}", new_doc);

//     Ok(())
// }
