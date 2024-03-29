use diesel::prelude::*;
use diesel_async::{AsyncConnection, AsyncPgConnection};
use dotenvy::dotenv;
use std::env;

pub async fn establish_connection() -> AsyncPgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    AsyncPgConnection::establish(&database_url).await.unwrap()
}