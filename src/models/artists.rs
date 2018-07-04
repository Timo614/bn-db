use db::connections::Connectable;
use diesel;
use diesel::prelude::*;
use schema::artists;
use utils::errors::DatabaseError;
use utils::errors::ErrorCode;
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
    pub fn commit(&self, conn: &Connectable) -> Result<Artist, DatabaseError> {
        DatabaseError::wrap(
            ErrorCode::InsertError,
            "Could not create new artist",
            diesel::insert_into(artists::table)
                .values(self)
                .get_result(conn.get_connection()),
        )
    }
}

impl Artist {
    pub fn create(name: &str) -> NewArtist {
        NewArtist {
            name: String::from(name),
        }
    }
}
