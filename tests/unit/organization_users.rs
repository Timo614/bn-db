use bigneon_db::models::{Organization, OrganizationUser, User};
use support::database;

#[test]
fn create_succeeds() {
    let test_connection = database::establish_test_connection();
    let user = User::new("Jeff", "jeff@tari.com", "555-555-5555", "examplePassword")
        .unwrap()
        .create(&test_connection);
    let user2 = User::new("Dan", "dan@tari.com", "555-555-5555", "examplePassword")
        .unwrap()
        .create(&test_connection);
    let organization = Organization::new(user.id).unwrap().create(&test_connection);
    let organization_user = OrganizationUser::new(organization.id, user2.id)
        .unwrap()
        .create(&test_connection);

    assert_eq!(organization_user.user_id, user2.id);
    assert_eq!(organization_user.organization_id, organization.id);
    assert_eq!(organization_user.id.to_string().is_empty(), false);
}
