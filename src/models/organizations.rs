use db::Connectable;
use diesel;
use diesel::prelude::*;
use models::{OrganizationUser, User};
use schema::{organizations, users};
use utils::errors::DatabaseError;
use utils::errors::ErrorCode;
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
    pub fn commit(&self, conn: &Connectable) -> Result<Organization, DatabaseError> {
        DatabaseError::wrap(
            ErrorCode::InsertError,
            "Could not create new organization",
            diesel::insert_into(organizations::table)
                .values(self)
                .get_result(conn.get_connection()),
        )
    }
}

impl Organization {
    pub fn create(owner_user_id: Uuid) -> NewOrganization {
        NewOrganization {
            owner_user_id: owner_user_id,
        }
    }

    pub fn users(&self, conn: &Connectable) -> Vec<User> {
        let organization_users = OrganizationUser::belonging_to(self);
        let organization_owner = users::table
            .find(self.owner_user_id)
            .first::<User>(conn.get_connection())
            .expect("Error loading organization owner");
        let mut users = organization_users
            .inner_join(users::table)
            .filter(users::id.ne(self.owner_user_id))
            .select(users::all_columns)
            .load::<User>(conn.get_connection())
            .expect("Error loading organization users");

        users.insert(0, organization_owner);
        users
    }
}
