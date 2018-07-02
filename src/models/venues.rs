use diesel;
use diesel::prelude::*;
use schema::venues;
use utils::errors::DatabaseError;
use uuid::Uuid;

#[derive(Associations, Identifiable, Queryable)]
pub struct Venue {
    pub id: Uuid,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "venues"]
pub struct NewVenue {
    pub name: String,
}

impl NewVenue {
    pub fn create(&self, connection: &PgConnection) -> Venue {
        diesel::insert_into(venues::table)
            .values(self)
            .get_result(connection)
            .expect("Error creating new venue")
    }
}

impl Venue {
    pub fn new(name: &str) -> Result<NewVenue, DatabaseError> {
        Ok(NewVenue {
            name: String::from(name),
        })
    }
}
