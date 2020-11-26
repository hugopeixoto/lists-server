use actix_web::{web, Error, HttpResponse};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db;
use crate::http;
use crate::models;
use crate::schema;

#[derive(Serialize, Deserialize)]
pub struct ItemForm {
    name: String,
}

fn create(
    collection_id: Uuid,
    item: ItemForm,
    connection: &db::Connection,
) -> db::Result<models::Item> {
    use schema::items;

    let item = diesel::insert_into(items::table)
        .values((
            items::name.eq(item.name),
            items::collection_id.eq(collection_id),
            items::created_at.eq(diesel::dsl::now),
            items::updated_at.eq(diesel::dsl::now),
        ))
        .get_result::<models::Item>(connection)?;

    Ok(item)
}

pub async fn run(
    pool: web::Data<db::Pool>,
    collection_id: web::Path<Uuid>,
    item: web::Json<ItemForm>,
) -> Result<HttpResponse, Error> {
    let collection_id = collection_id.into_inner();
    let conn = pool.get().map_err(http::internal_server_error)?;
    let item = item.0;

    let item = web::block(move || create(collection_id, item, &conn))
        .await
        .map_err(http::internal_server_error)?;

    let json = serde_json::to_string(&item).map_err(http::internal_server_error)?;

    Ok(HttpResponse::Ok().body(json))
}
