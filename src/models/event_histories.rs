use diesel;
use diesel::prelude::*;
use models::{Event, Order, User};
use schema::event_histories;
use utils::errors::DatabaseError;
use uuid::Uuid;

#[derive(Associations, Identifiable, Queryable)]
#[belongs_to(Event)]
#[belongs_to(Order)]
#[belongs_to(User)]
#[table_name = "event_histories"]
pub struct EventHistory {
    pub id: Uuid,
    pub event_id: Uuid,
    pub order_id: Uuid,
    pub user_id: Uuid,
    pub protocol_reference_hash: String,
}

#[derive(Insertable)]
#[table_name = "event_histories"]
pub struct NewEventHistory {
    pub event_id: Uuid,
    pub order_id: Uuid,
    pub user_id: Uuid,
    pub protocol_reference_hash: String,
}

impl NewEventHistory {
    pub fn create(&self, connection: &PgConnection) -> EventHistory {
        diesel::insert_into(event_histories::table)
            .values(self)
            .get_result(connection)
            .expect("Error creating new event history")
    }
}

impl EventHistory {
    pub fn new(
        event_id: Uuid,
        order_id: Uuid,
        user_id: Uuid,
        protocol_reference_hash: &str,
    ) -> Result<NewEventHistory, DatabaseError> {
        Ok(NewEventHistory {
            event_id: event_id,
            order_id: order_id,
            user_id: user_id,
            protocol_reference_hash: String::from(protocol_reference_hash),
        })
    }
}
