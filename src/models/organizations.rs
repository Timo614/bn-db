use diesel;
use diesel::prelude::*;
use models::{OrganizationUser, User};
use schema::{organizations, users};
use utils::errors::DatabaseError;
use uuid::Uuid;

#[derive(Associations, Identifiable, Queryable)]
#[belongs_to(User, foreign_key = "owner_user_id")]
pub struct Organization {
    pub id: Uuid,
    pub owner_user_id: Uuid,
}

#[derive(Insertable)]
#[table_name = "organizations"]
pub struct NewOrganization {
    pub owner_user_id: Uuid,
}

impl NewOrganization {
    pub fn create(&self, connection: &PgConnection) -> Organization {
        diesel::insert_into(organizations::table)
            .values(self)
            .get_result(connection)
            .expect("Error creating new organization")
    }
}

impl Organization {
    pub fn new(owner_user_id: Uuid) -> Result<NewOrganization, DatabaseError> {
        Ok(NewOrganization {
            owner_user_id: owner_user_id,
        })
    }

    pub fn users(&self, connection: &PgConnection) -> Vec<User> {
        let organization_users = OrganizationUser::belonging_to(self);
        let organization_owner = users::table
            .find(self.owner_user_id)
            .first::<User>(connection)
            .expect("Error loading organization owner");
        let mut users = organization_users
            .inner_join(users::table)
            .filter(users::id.ne(self.owner_user_id))
            .select(users::all_columns)
            .load::<User>(connection)
            .expect("Error loading organization users");

        users.insert(0, organization_owner);
        users
    }
}
