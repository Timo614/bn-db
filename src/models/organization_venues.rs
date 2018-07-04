use db::Connectable;
use diesel;
use diesel::prelude::*;
use models::{Organization, Venue};
use schema::organization_venues;
use utils::errors::DatabaseError;
use utils::errors::ErrorCode;
use uuid::Uuid;

#[derive(Associations, Identifiable, Queryable)]
#[belongs_to(Venue)]
#[belongs_to(Organization)]
#[table_name = "organization_venues"]
pub struct OrganizationVenue {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub venue_id: Uuid,
}

#[derive(Insertable)]
#[table_name = "organization_venues"]
pub struct NewOrganizationVenue {
    pub organization_id: Uuid,
    pub venue_id: Uuid,
}

impl NewOrganizationVenue {
    pub fn commit(&self, conn: &Connectable) -> Result<OrganizationVenue, DatabaseError> {
        DatabaseError::wrap(
            ErrorCode::InsertError,
            "Could not create new organization venue",
            diesel::insert_into(organization_venues::table)
                .values(self)
                .get_result(conn.get_connection()),
        )
    }
}

impl OrganizationVenue {
    pub fn create(organization_id: Uuid, venue_id: Uuid) -> NewOrganizationVenue {
        NewOrganizationVenue {
            organization_id: organization_id,
            venue_id: venue_id,
        }
    }
}
