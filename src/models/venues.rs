use db::Connectable;
use diesel;
use diesel::prelude::*;
use schema::venues;
use utils::errors::DatabaseError;
use utils::errors::ErrorCode;
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
    pub fn commit(&self, connection: &Connectable) -> Result<Venue, DatabaseError> {
        DatabaseError::wrap(
            ErrorCode::InsertError,
            "Could not create new venue",
            diesel::insert_into(venues::table)
                .values(self)
                .get_result(connection.get_connection()),
        )
    }
}

impl Venue {
    pub fn create(name: &str) -> NewVenue {
        NewVenue {
            name: String::from(name),
        }
    }
}
