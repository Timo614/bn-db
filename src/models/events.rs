use chrono::NaiveDateTime;
use db::Connectable;
use diesel;
use diesel::prelude::*;
use models::{Organization, Venue};
use schema::events;
use utils::errors::DatabaseError;
use utils::errors::ErrorCode;
use uuid::Uuid;

#[derive(Associations, Identifiable, Queryable, AsChangeset)]
#[belongs_to(Organization)]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[belongs_to(Venue)]
#[table_name = "events"]
pub struct Event {
    pub id: Uuid,
    pub name: String,
    pub organization_id: Uuid,
    pub venue_id: Uuid,
    pub created_at: NaiveDateTime,
    pub ticket_sell_date: NaiveDateTime,
    pub event_start: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "events"]
pub struct NewEvent {
    pub name: String,
    pub organization_id: Uuid,
    pub venue_id: Uuid,
    pub event_start: NaiveDateTime,
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
    pub fn create(
        name: &str,
        organization_id: Uuid,
        venue_id: Uuid,
        event_start: NaiveDateTime,
    ) -> NewEvent {
        NewEvent {
            name: name.into(),
            organization_id: organization_id,
            venue_id: venue_id,
            event_start: event_start,
        }
    }
    pub fn update(&self, conn: &Connectable) -> Result<Event, DatabaseError> {
        DatabaseError::wrap(
            ErrorCode::UpdateError,
            "Could not update event",
            diesel::update(self)
                .set(self)
                .get_result(conn.get_connection()),
        )
    }
    pub fn find(id: &Uuid, conn: &Connectable) -> Result<Event, DatabaseError> {
        DatabaseError::wrap(
            ErrorCode::QueryError,
            "Error loading event",
            events::table.find(id).first::<Event>(conn.get_connection()),
        )
    }
    pub fn find_all_events_from_venue(
        venue_id: &Uuid,
        conn: &Connectable,
    ) -> Result<Vec<Event>, DatabaseError> {
        DatabaseError::wrap(
            ErrorCode::QueryError,
            "Error loading event via venue",
            events::table
                .filter(events::venue_id.eq(venue_id))
                .load(conn.get_connection()),
        )
    }
    pub fn find_all_events_from_organization(
        organization_id: &Uuid,
        conn: &Connectable,
    ) -> Result<Vec<Event>, DatabaseError> {
        DatabaseError::wrap(
            ErrorCode::QueryError,
            "Error loading events via organization",
            events::table
                .filter(events::organization_id.eq(organization_id))
                .load(conn.get_connection()),
        )
    }

    pub fn all(conn: &Connectable) -> Result<Vec<Event>, DatabaseError> {
        DatabaseError::wrap(
            ErrorCode::QueryError,
            "Unable to load all events",
            events::table
                .order_by(events::event_start.desc())
                .load(conn.get_connection()),
        )
    }
}
