use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::models::*;
use crate::schema::*;

pub struct RustaceanRepository {}

impl RustaceanRepository {
    pub async fn find(c: &mut AsyncPgConnection, id: i32) -> QueryResult<Rustacean> {
        rustaceans::table.find(id).get_result(c).await
    }

    pub async fn find_mutliple(
        c: &mut AsyncPgConnection,
        limit: i64,
    ) -> QueryResult<Vec<Rustacean>> {
        rustaceans::table.limit(limit).get_results(c).await
    }

    pub async fn create(
        c: &mut AsyncPgConnection,
        new_rustacean: NewRustacean,
    ) -> QueryResult<Rustacean> {
        diesel::insert_into(rustaceans::table)
            .values(new_rustacean)
            .get_result(c)
            .await
    }

    pub async fn update(c: &mut AsyncPgConnection, rustacean: Rustacean) -> QueryResult<Rustacean> {
        diesel::update(rustaceans::table.find(rustacean.id))
            .set((
                rustaceans::name.eq(rustacean.name),
                rustaceans::email.eq(rustacean.email),
            ))
            .get_result(c)
            .await
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(rustaceans::table.find(id)).execute(c).await
    }
}

pub struct CrateRepository {}

impl CrateRepository {
    pub async fn find(c: &mut AsyncPgConnection, id: i32) -> QueryResult<Crate> {
        crates::table.find(id).get_result(c).await
    }

    pub async fn find_multiple(c: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<Crate>> {
        crates::table.limit(limit).get_results(c).await
    }

    pub async fn create(c: &mut AsyncPgConnection, new_crate: NewCrate) -> QueryResult<Crate> {
        diesel::insert_into(crates::table)
            .values(new_crate)
            .get_result(c)
            .await
    }

    pub async fn update(c: &mut AsyncPgConnection, crat: Crate) -> QueryResult<Crate> {
        diesel::update(crates::table.find(crat.id))
            .set((
                crates::rustacean_id.eq(crat.rustacean_id),
                crates::code.eq(crat.code),
                crates::name.eq(crat.name),
                crates::version.eq(crat.version),
                crates::description.eq(crat.description),
            ))
            .get_result(c)
            .await
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(crates::table.find(id)).execute(c).await
    }
}
