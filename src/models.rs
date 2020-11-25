use std::time::SystemTime;
use uuid::Uuid;

use crate::schema::{collections, items};

#[derive(Identifiable, Queryable)]
#[table_name = "collections"]
pub struct Collection {
    pub id: Uuid,
    pub name: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Collection)]
#[table_name = "items"]
pub struct Item {
    pub id: Uuid,
    pub collection_id: Uuid,
    pub name: String,
    pub visible: bool,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}
