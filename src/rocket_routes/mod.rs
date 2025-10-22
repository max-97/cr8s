use std::error::Error;

use rocket::{
    Request,
    http::Status,
    outcome::Outcome,
    request::{self, FromRequest},
    response::status::Custom,
};
use rocket_db_pools::Connection;
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use serde_json::json;

use crate::{models::User, repositories::UserRepository};

pub mod authorization;
pub mod crates;
pub mod rustaceans;

#[derive(rocket_db_pools::Database)]
#[database("postgres")]
pub struct DbConn(rocket_db_pools::diesel::PgPool);

#[derive(rocket_db_pools::Database)]
#[database("redis")]
pub struct CacheConn(rocket_db_pools::deadpool_redis::Pool);

pub fn server_error(e: Box<dyn Error>) -> Custom<serde_json::Value> {
    rocket::error!("{}", e);
    Custom(Status::InternalServerError, json!("Error"))
}

pub fn server_error_404(e: Box<dyn Error>) -> Custom<serde_json::Value> {
    rocket::error!("{}", e);
    Custom(Status::NotFound, json!("Error: Not Found"))
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let session_header = req
            .headers()
            .get_one("Authorization")
            .map(|v| v.split_whitespace().collect::<Vec<_>>())
            .filter(|v| v.len() == 2 && v[0] == "Bearer");

        if let Some(header_value) = session_header {
            let mut cache = req
                .guard::<Connection<CacheConn>>()
                .await
                .expect("Can not connect to Redis in request quard");
            let mut db = req
                .guard::<Connection<DbConn>>()
                .await
                .expect("Can not connect to Postgres in request quard");

            let result = cache
                .get::<String, i32>(format!("session/{}", header_value[1]))
                .await;

            if let Ok(user_id) = result
                && let Ok(user) = UserRepository::find(&mut db, user_id).await
            {
                return Outcome::Success(user);
            }
        }

        Outcome::Error((Status::Unauthorized, ()))
    }
}
