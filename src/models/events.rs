use diesel;
use diesel::prelude::*;
use models::{Organization, Venue};
use schema::events;
use utils::errors::DatabaseError;
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
    pub fn create(&self, connection: &PgConnection) -> Event {
        diesel::insert_into(events::table)
            .values(self)
            .get_result(connection)
            .expect("Error creating new event")
    }
}

impl Event {
    pub fn new(organization_id: Uuid, venue_id: Uuid) -> Result<NewEvent, DatabaseError> {
        Ok(NewEvent {
            organization_id: organization_id,
            venue_id: venue_id,
        })
    }
}
