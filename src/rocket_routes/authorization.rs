use argon2::{PasswordHash, PasswordVerifier};
use rocket::{
    response::status::Custom,
    serde::json::{Json, Value, json},
};
use rocket_db_pools::Connection;

use crate::{
    repositories::UserRepository,
    rocket_routes::{DbConn, server_error, server_error_404},
};

#[derive(serde::Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[rocket::post("/login", format = "json", data = "<credentials>")]
pub async fn login(
    mut db: Connection<DbConn>,
    credentials: Json<Credentials>,
) -> Result<Value, Custom<Value>> {
    UserRepository::find_by_username(&mut db, &credentials.username)
        .await
        .map(|user| {
            let argon2 = argon2::Argon2::default();
            let db_hash = PasswordHash::new(&user.password).unwrap();
            match argon2.verify_password(credentials.password.as_bytes(), &db_hash) {
                Ok(_) => json!("Success"),
                Err(_) => json!("Unauthorized"),
            }
        })
        .map_err(|e| server_error(e.into()))
}
