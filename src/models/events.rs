use db::Connectable;
use diesel;
use diesel::prelude::*;
use models::{Organization, Venue};
use schema::events;
use utils::errors::DatabaseError;
use utils::errors::ErrorCode;
use uuid::Uuid;

#[derive(Associations, Identifiable, Queryable)]
#[belongs_to(Organization)]
#[belongs_to(Venue)]
pub struct Event {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub venue_id: Uuid,
}

#[derive(Insertable)]
#[table_name = "events"]
pub struct NewEvent {
    pub organization_id: Uuid,
    pub venue_id: Uuid,
}

impl NewEvent {
    pub fn commit(&self, conn: &Connectable) -> Result<Event, DatabaseError> {
        DatabaseError::wrap(
            ErrorCode::InsertError,
            "Could not create new event",
            diesel::insert_into(events::table)
                .values(self)
                .get_result(conn.get_connection()),
        )
    }
}

impl Event {
    pub fn create(organization_id: Uuid, venue_id: Uuid) -> NewEvent {
        NewEvent {
            organization_id: organization_id,
            venue_id: venue_id,
        }
    }
}
