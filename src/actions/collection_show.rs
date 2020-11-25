use actix_web::{web, Error, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;

use crate::models;
use crate::schema;
use crate::db;
use crate::http;

struct View {
    collection: models::Collection,
    items: Vec<models::Item>,
}

fn fetch(user_id: Uuid, connection: &db::Connection) -> db::Result<Option<View>> {
    use schema::collections as collections;
    use schema::items as items;

    let collection = collections::table
        .find(user_id)
        .first::<models::Collection>(connection)
        .optional()?;

    if let Some(collection) = collection {
        let items = items::table
            .filter(items::collection_id.eq(collection.id))
            .load::<models::Item>(connection)?;

        Ok(Some(View { collection, items }))
    } else {
        Ok(None)
    }
}

pub async fn run(
    pool: web::Data<db::Pool>,
    user_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let user_id = user_id.into_inner();
    let conn = pool.get().map_err(http::internal_server_error)?;

    // look at me, blocking everything to make my life easier.
    let r = web::block(move || { fetch(user_id, &conn) })
        .await
        .map_err(http::internal_server_error)?;

    if let Some(view) = r {
        Ok(HttpResponse::Ok()
           .body(format!("name: {}, items: {}", view.collection.name, view.items.len())))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}
