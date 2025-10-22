use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::response::status::NoContent;
use rocket::serde::json::Json;
use rocket::serde::json::{Value, json};
use rocket_db_pools::Connection;

use crate::models::User;
use crate::models::{NewRustacean, Rustacean};
use crate::repositories::RustaceanRepository;
use crate::rocket_routes::{DbConn, server_error, server_error_404};

#[rocket::get("/rustaceans")]
pub async fn get_rustaceans(
    mut db: Connection<DbConn>,
    _user: User,
) -> Result<Value, Custom<Value>> {
    RustaceanRepository::find_multiple(&mut db, 100)
        .await
        .map(|rustaceans| json!(rustaceans))
        .map_err(|e| server_error(e.into()))
}

#[rocket::get("/rustaceans/<id>")]
pub async fn get_rustacean(
    mut db: Connection<DbConn>,
    id: i32,
    _user: User,
) -> Result<Value, Custom<Value>> {
    RustaceanRepository::find(&mut db, id)
        .await
        .map(|rustaceans| json!(rustaceans))
        .map_err(|e| match e {
            diesel::result::Error::NotFound => server_error_404(e.into()),
            _ => server_error(e.into()),
        })
}

#[rocket::post("/rustaceans", format = "json", data = "<new_rustacean>")]
pub async fn create_rustacean(
    mut db: Connection<DbConn>,
    new_rustacean: Json<NewRustacean>,
    _user: User,
) -> Result<Custom<Value>, Custom<Value>> {
    RustaceanRepository::create(&mut db, new_rustacean.into_inner())
        .await
        .map(|rustaceans| Custom(Status::Created, json!(rustaceans)))
        .map_err(|e| server_error(e.into()))
}

#[rocket::put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
pub async fn update_rustacean(
    mut db: Connection<DbConn>,
    id: i32,
    rustacean: Json<Rustacean>,
    _user: User,
) -> Result<Value, Custom<Value>> {
    RustaceanRepository::update(&mut db, id, rustacean.into_inner())
        .await
        .map(|rustaceans| json!(rustaceans))
        .map_err(|e| server_error(e.into()))
}

#[rocket::delete("/rustaceans/<id>")]
pub async fn delete_rustacean(
    mut db: Connection<DbConn>,
    id: i32,
    _user: User,
) -> Result<NoContent, Custom<Value>> {
    RustaceanRepository::delete(&mut db, id)
        .await
        .map(|_| NoContent)
        .map_err(|e| server_error(e.into()))
}
