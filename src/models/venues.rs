use db::Connectable;
use diesel;
use diesel::prelude::*;
use models::OrganizationVenue;
use schema::venues;
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

    //todo refactor function
    pub fn find_all_for_organization(
        organization_id: &Uuid,
        conn: &Connectable,
    ) -> Result<Vec<Venue>, DatabaseError> {
        let all_linked_organization_ids =
            OrganizationVenue::find_via_organization_all(&organization_id, conn).unwrap();
        let mut found_venues: Vec<Venue> = Vec::new();
        let mut wrapped_results: Result<Vec<Venue>, DatabaseError> = Ok(Vec::new());
        for i in 0..all_linked_organization_ids.len() {
            let temp_venue = Venue::find(&all_linked_organization_ids[i].venue_id, conn);
            match temp_venue {
                Ok(val) => found_venues.push(val),
                Err(e) => wrapped_results = Err(e),
            }
        }
        match wrapped_results {
            Err(e) => wrapped_results = Err(e), //some error found wrapping in erropr
            _ => wrapped_results = Ok(found_venues), //no error found, returning result
        }
        DatabaseError::wrap(
            ErrorCode::QueryError,
            "Error loading events via organization",
            wrapped_results,
        )
    }

    pub fn add_to_organization(
        &self,
        organization_id: &Uuid,
        conn: &Connectable,
    ) -> Result<OrganizationVenue, DatabaseError> {
        DatabaseError::wrap(
            ErrorCode::UpdateError,
            "Could not update venue",
            OrganizationVenue::create(*organization_id, self.id).commit(conn),
        )
    }
}
