use db::Connectable;
use diesel;
use diesel::prelude::*;
use models::OrganizationVenue;
use schema::{organization_venues, venues};
use utils::errors::DatabaseError;
use utils::errors::ErrorCode;
use uuid::Uuid;

#[derive(Associations, Identifiable, Queryable, AsChangeset, Serialize, Deserialize, PartialEq,
         Debug)]
#[table_name = "venues"]
pub struct Venue {
    pub id: Uuid,
    pub name: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub zip: Option<String>,
    pub phone: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize, PartialEq, Debug)]
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
    pub fn update(&self, conn: &Connectable) -> Result<Venue, DatabaseError> {
        DatabaseError::wrap(
            ErrorCode::UpdateError,
            "Could not update venue",
            diesel::update(self)
                .set(self)
                .get_result(conn.get_connection()),
        )
    }
    pub fn find(id: &Uuid, conn: &Connectable) -> Result<Venue, DatabaseError> {
        DatabaseError::wrap(
            ErrorCode::QueryError,
            "Error loading veue",
            venues::table.find(id).first::<Venue>(conn.get_connection()),
        )
    }
    pub fn all(conn: &Connectable) -> Result<Vec<Venue>, DatabaseError> {
        DatabaseError::wrap(
            ErrorCode::QueryError,
            "Unable to load all venues",
            venues::table.load(conn.get_connection()),
        )
    }

    pub fn find_for_organization(
        organization_id: Uuid,
        conn: &Connectable,
    ) -> Result<Vec<Venue>, DatabaseError> {
        DatabaseError::wrap(
            ErrorCode::QueryError,
            "Could not retrieve venues",
            organization_venues::table
                .filter(organization_venues::organization_id.eq(organization_id))
                .inner_join(venues::table)
                .select(venues::all_columns)
                .load::<Venue>(conn.get_connection()),
        )
    }

    pub fn add_to_organization(
        &self,
        organization_id: &Uuid,
        conn: &Connectable,
    ) -> Result<OrganizationVenue, DatabaseError> {
        OrganizationVenue::create(*organization_id, self.id)
            .commit(conn)
            .map_err(|e| {
                DatabaseError::new(
                    ErrorCode::UpdateError,
                    Some(&format!("Could not update venue:{}", e.cause.unwrap())),
                )
            })
    }
}
