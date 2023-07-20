use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;
use std::sync::Arc;

pub type RtfDb = Arc<Pool<Postgres>>;
pub type MqDb = Arc<Pool<Postgres>>;

pub async fn rtf_db_connect() -> Pool<Postgres> {
	dotenv().ok();
	let db_url = env::var("RTF_DATABASE_URL").expect("RTF_DATABASE_URL must be set");
	PgPoolOptions::new()
		.max_connections(50)
		.connect(&db_url)
		.await
		.expect("Error connecting to database")
}

pub async fn mq_db_connect() -> Pool<Postgres> {
	dotenv().ok();
	let db_url = env::var("MQ_DATABASE_URL").expect("MQ_DATABASE_URL must be set");
	PgPoolOptions::new()
		.max_connections(50)
		.connect(&db_url)
		.await
		.expect("Error connecting to database")
}