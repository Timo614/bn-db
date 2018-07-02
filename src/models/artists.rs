use diesel;
use diesel::prelude::*;
use schema::artists;
use utils::errors::DatabaseError;
use uuid::Uuid;

#[derive(Associations, Identifiable, Queryable)]
pub struct Artist {
    pub id: Uuid,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "artists"]
pub struct NewArtist {
    pub name: String,
}

impl NewArtist {
    pub fn create(&self, connection: &PgConnection) -> Artist {
        diesel::insert_into(artists::table)
            .values(self)
            .get_result(connection)
            .expect("Error creating new artist")
    }
}

impl Artist {
    pub fn new(name: &str) -> Result<NewArtist, DatabaseError> {
        Ok(NewArtist {
            name: String::from(name),
        })
    }
}
