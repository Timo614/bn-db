use db::connections::Connectable;
use diesel;
use diesel::prelude::*;
use schema::artists;
use utils::errors::DatabaseError;
use utils::errors::ErrorCode;
use uuid::Uuid;

#[derive(Associations, Identifiable, Queryable, Serialize)]
pub struct Artist {
    pub id: Uuid,
    pub name: String,
}

#[derive(Insertable, Deserialize)]
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

    pub fn all(conn: &Connectable) -> Vec<Artist> {
        artists::table.load(conn.get_connection()).expect("Unable to load artists")
    }

    pub fn find(id: &Uuid, conn: &Connectable) -> Artist {
        artists::table.find(id).first::<Artist>(conn.get_connection()).expect("Error loading artist")
    }

    pub fn update_attributes(&self, artist_parameters: &NewArtist, conn: &Connectable) -> Artist {
        use schema::artists::dsl::*;
        diesel::update(artists.filter(id.eq(self.id)))
            .set(name.eq(&artist_parameters.name))
            .get_result(conn.get_connection())
            .expect("Error updating artist")
    }

    pub fn destroy(&self, conn: &Connectable) -> bool {
        use schema::artists::dsl::*;
        let deleted_count = diesel::delete(artists.filter(id.eq(self.id))).execute(conn.get_connection()).expect("Failed to destroy artist record");
        deleted_count > 0
    }
}
