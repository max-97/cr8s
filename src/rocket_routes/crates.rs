use rocket::http::Status;
use rocket::response::status::{Custom, NoContent};
use rocket::serde::json::{Json, Value, json};
use rocket_db_pools::Connection;

use crate::models::{Crate, NewCrate, User};
use crate::repositories::CrateRepository;
use crate::rocket_routes::{DbConn, server_error, server_error_404};

#[rocket::get("/crates")]
pub async fn get_crates(mut db: Connection<DbConn>, _user: User) -> Result<Value, Custom<Value>> {
    CrateRepository::find_multiple(&mut db, 100)
        .await
        .map(|crates| json!(crates))
        .map_err(|e| server_error(e.into()))
}

#[rocket::get("/crates/<id>")]
pub async fn get_crate(
    mut db: Connection<DbConn>,
    id: i32,
    _user: User,
) -> Result<Value, Custom<Value>> {
    CrateRepository::find(&mut db, id)
        .await
        .map(|crates| json!(crates))
        .map_err(|e| match e {
            diesel::result::Error::NotFound => server_error_404(e.into()),
            _ => server_error(e.into()),
        })
}

#[rocket::post("/crates", format = "json", data = "<new_crate>")]
pub async fn create_crate(
    mut db: Connection<DbConn>,
    new_crate: Json<NewCrate>,
    _user: User,
) -> Result<Custom<Value>, Custom<Value>> {
    CrateRepository::create(&mut db, new_crate.into_inner())
        .await
        .map(|crates| Custom(Status::Created, json!(crates)))
        .map_err(|e| server_error(e.into()))
}

#[rocket::put("/crates/<id>", format = "json", data = "<a_crate>")]
pub async fn update_crate(
    mut db: Connection<DbConn>,
    id: i32,
    a_crate: Json<Crate>,
    _user: User,
) -> Result<Value, Custom<Value>> {
    CrateRepository::update(&mut db, id, a_crate.into_inner())
        .await
        .map(|crates| json!(crates))
        .map_err(|e| server_error(e.into()))
}

#[rocket::delete("/crates/<id>")]
pub async fn delete_crate(
    mut db: Connection<DbConn>,
    id: i32,
    _user: User,
) -> Result<NoContent, Custom<Value>> {
    CrateRepository::delete(&mut db, id)
        .await
        .map(|_| NoContent)
        .map_err(|e| server_error(e.into()))
}
