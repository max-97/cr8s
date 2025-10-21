use rocket::{
    response::status::Custom,
    serde::json::{Json, Value, json},
};
use rocket_db_pools::Connection;

use crate::{
    auth::{Credentials, authorize_user},
    repositories::UserRepository,
    rocket_routes::{DbConn, server_error},
};

#[rocket::post("/login", format = "json", data = "<credentials>")]
pub async fn login(
    mut db: Connection<DbConn>,
    credentials: Json<Credentials>,
) -> Result<Value, Custom<Value>> {
    UserRepository::find_by_username(&mut db, &credentials.username)
        .await
        .map(
            |user| match authorize_user(&user, credentials.into_inner()) {
                Ok(token) => json!(token),
                Err(_) => json!("Unauthorized"),
            },
        )
        .map_err(|e| server_error(e.into()))
}
