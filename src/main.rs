mod adapters;
mod libs;


use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use db::sql::create_sql_db;
use dotenv::dotenv;

use libs::randomness::{NumberGenerator, SmallRngGenerator};
use log::{error};
use rand::distributions::uniform::{SampleUniform};

use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{RangeInclusive};


use tokio::{task::JoinSet, time::Instant};

type QueryType = i32;

struct State<T: SampleUniform> {
    db: DatabaseConnection,
    num_requests: u32,
    range: RangeInclusive<T>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct RandomNumber<T: SampleUniform> {
    pub json: T,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let db = create_sql_db().await.expect("Db is needed!");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(State {
                db: db.to_owned(),
                num_requests: 20,
                range: 0..=10i32,
            }))
            .service(index)
            .service(run)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Adinmo test; go to /run to start.")
}

#[get("/run")]
async fn run(state: web::Data<State<QueryType>>) -> Result<String, libs::error::ErrorResponder> {
    println!("running");

    let random_numbers: Vec<QueryType> =
        SmallRngGenerator::<QueryType>::get_random_numbers_inclusive(
            state.num_requests,
            &state.range,
        );

    let t = Instant::now();
    let set = RandomNumberHandler::<QueryType>::send_posties(random_numbers, None);
    let all_numbers = RandomNumberHandler::<QueryType>::handle_responses(set).await;

    let elapsed_ms = t.elapsed().subsec_millis().to_string();
    let post_batch_id =
        adapters::post_batch::PostBatchAdapter::insert(&state.db, elapsed_ms).await?;

    for num in &all_numbers {
        adapters::random_number::RandomNumberAdapter::insert(&state.db, *num, post_batch_id)
            .await?;
    }

    let mut most_common = RandomNumberHandler::<QueryType>::get_most_common_numbers(&all_numbers);
    most_common.sort();
    Ok(serde_json::to_string(&most_common)?)
}

mod entities;


pub struct RandomNumberHandler<T>(T);
impl<T> RandomNumberHandler<T>
where
    for<'a> T: Clone
        + Copy
        + 'static
        + Eq
        + PartialEq
        + Hash
        + Serialize
        + Deserialize<'a>
        + Debug
        + SampleUniform,
{
    pub fn send_posties(
        random_numbers: Vec<T>,
        limit: Option<usize>,
    ) -> JoinSet<Result<reqwest::Response, reqwest::Error>> {
        let client: reqwest::Client = reqwest::Client::new();
        let mut set = JoinSet::new();
        (0usize..limit.unwrap_or(random_numbers.len())).for_each(|i| {
            let request_builder = client
                .post("https://httpbin.org/post")
                .json(&random_numbers[i]);
            if let Ok(request) = request_builder.build() {
                // save request to db??
                set.spawn(client.execute(request));
            } else {
                error!("Could not build request.")
            }
        });
        set
    }

    pub async fn handle_responses(
        mut set: JoinSet<Result<reqwest::Response, reqwest::Error>>,
    ) -> Vec<T> {
        // Save new post batch to sql
        let mut out = Vec::new();
        while let Some(maybe_response) = set.join_next().await {
            match maybe_response {
                Ok(Ok(response)) => {
                    match response.status() {
                        reqwest::StatusCode::OK => match response.json::<RandomNumber<T>>().await {
                            Ok(rn) => {
                                out.push(rn.json);
                            }
                            Err(e) => {
                                error!("{:?}", e);
                            }
                        },
                        _ => {
                            error!("Status response was not OK!");
                        }
                    };
                }
                Ok(Err(e)) => {
                    error!("{:?}", e);
                }
                Err(e) => {
                    error!("{:?}", e);
                }
            }
        }

        out
    }

    pub fn get_most_common_numbers(all_numbers: &[T]) -> Vec<T> {
        let mut most_common: HashMap<T, u32> = HashMap::new();
        all_numbers.iter().for_each(|i| {
            let counter: u32 = *most_common.get(i).unwrap_or(&Default::default());
            most_common.insert(*i, counter + 1);
        });
        println!("{:?}", &most_common);
        most_common
            .into_iter()
            .filter(|(_key, count)| count > &1)
            .map(|(key, _v)| key)
            .collect::<Vec<T>>()
    }
}
