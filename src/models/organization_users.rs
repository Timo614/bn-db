use diesel;
use diesel::prelude::*;
use models::{Organization, User};
use schema::organization_users;
use utils::errors::DatabaseError;
use uuid::Uuid;

#[derive(Associations, Identifiable, Queryable)]
#[belongs_to(User)]
#[belongs_to(Organization)]
#[table_name = "organization_users"]
pub struct OrganizationUser {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Insertable)]
#[table_name = "organization_users"]
pub struct NewOrganizationUser {
    pub organization_id: Uuid,
    pub user_id: Uuid,
}

impl NewOrganizationUser {
    pub fn create(&self, connection: &PgConnection) -> OrganizationUser {
        diesel::insert_into(organization_users::table)
            .values(self)
            .get_result(connection)
            .expect("Error creating new organization user")
    }
}

impl OrganizationUser {
    pub fn new(organization_id: Uuid, user_id: Uuid) -> Result<NewOrganizationUser, DatabaseError> {
        Ok(NewOrganizationUser {
            organization_id: organization_id,
            user_id: user_id,
        })
    }
}
