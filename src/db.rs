extern crate dotenvy;
use sqlx::{Sqlite, sqlite::SqlitePool, pool::PoolConnection};
use dotenvy::dotenv;

pub fn fetch_all(conn: &mut PoolConnection<Sqlite>) {

}