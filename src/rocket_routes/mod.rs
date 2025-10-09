use std::error::Error;

use rocket::{http::Status, response::status::Custom};
use serde_json::json;

pub mod authorization;
pub mod crates;
pub mod rustaceans;

#[derive(rocket_db_pools::Database)]
#[database("postgres")]
pub struct DbConn(rocket_db_pools::diesel::PgPool);

pub fn server_error(e: Box<dyn Error>) -> Custom<serde_json::Value> {
    rocket::error!("{}", e);
    Custom(Status::InternalServerError, json!("Error"))
}

pub fn server_error_404(e: Box<dyn Error>) -> Custom<serde_json::Value> {
    rocket::error!("{}", e);
    Custom(Status::NotFound, json!("Error: Not Found"))
}
