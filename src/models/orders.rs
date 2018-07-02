use diesel;
use diesel::prelude::*;
use models::{Event, User};
use schema::orders;
use utils::errors::DatabaseError;
use uuid::Uuid;

#[derive(Associations, Identifiable, Queryable)]
#[belongs_to(Event)]
#[belongs_to(User)]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub event_id: Uuid,
}

#[derive(Insertable)]
#[table_name = "orders"]
pub struct NewOrder {
    pub user_id: Uuid,
    pub event_id: Uuid,
}

impl NewOrder {
    pub fn create(&self, connection: &PgConnection) -> Order {
        use schema::orders;
        diesel::insert_into(orders::table)
            .values(self)
            .get_result(connection)
            .expect("Error creating new order")
    }
}

impl Order {
    pub fn new(user_id: Uuid, event_id: Uuid) -> Result<NewOrder, DatabaseError> {
        Ok(NewOrder {
            user_id: user_id,
            event_id: event_id,
        })
    }
}
