use db::Connectable;
use diesel;
use diesel::prelude::*;
use models::{Artist, Event};
use schema::event_artists;
use utils::errors::DatabaseError;
use uuid::Uuid;

#[derive(Associations, Identifiable, Queryable)]
#[belongs_to(Event)]
#[belongs_to(Artist)]
#[table_name = "event_artists"]
pub struct EventArtist {
    pub id: Uuid,
    pub event_id: Uuid,
    pub artist_id: Uuid,
    pub rank: i32,
}

#[derive(Insertable)]
#[table_name = "event_artists"]
pub struct NewEventArtist {
    pub event_id: Uuid,
    pub artist_id: Uuid,
    pub rank: i32,
}

impl NewEventArtist {
    pub fn create(&self, conn: &Connectable) -> EventArtist {
        diesel::insert_into(event_artists::table)
            .values(self)
            .get_result(conn.get_connection())
            .expect("Error creating new event artist")
    }
}

impl EventArtist {
    pub fn new(
        event_id: Uuid,
        artist_id: Uuid,
        rank: i32,
    ) -> Result<NewEventArtist, DatabaseError> {
        Ok(NewEventArtist {
            event_id: event_id,
            artist_id: artist_id,
            rank: rank,
        })
    }
}
