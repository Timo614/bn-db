use diesel;
use diesel::prelude::*;
use models::{Organization, Venue};
use schema::organization_venues;
use utils::errors::DatabaseError;
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
    pub fn create(&self, connection: &PgConnection) -> OrganizationVenue {
        diesel::insert_into(organization_venues::table)
            .values(self)
            .get_result(connection)
            .expect("Error creating new organization venue")
    }
}

impl OrganizationVenue {
    pub fn new(
        organization_id: Uuid,
        venue_id: Uuid,
    ) -> Result<NewOrganizationVenue, DatabaseError> {
        Ok(NewOrganizationVenue {
            organization_id: organization_id,
            venue_id: venue_id,
        })
    }
}
