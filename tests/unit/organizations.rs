use bigneon_db::models::{Organization, OrganizationUser, User};
use support::database;

#[test]
fn create_succeeds() {
    let test_connection = database::establish_test_connection();
    let user = User::new("Jeff", "jeff@tari.com", "555-555-5555", "examplePassword")
        .unwrap()
        .create(&test_connection);
    let organization = Organization::new(user.id).unwrap().create(&test_connection);

    assert_eq!(organization.owner_user_id, user.id);
    assert_eq!(organization.id.to_string().is_empty(), false);
}

#[test]
fn users_association() {
    let test_connection = database::establish_test_connection();
    let user = User::new("Jeff", "jeff@tari.com", "555-555-5555", "examplePassword")
        .unwrap()
        .create(&test_connection);
    let user2 = User::new("David", "david@tari.com", "555-555-5555", "examplePassword")
        .unwrap()
        .create(&test_connection);
    let user3 = User::new("Ann", "ann@tari.com", "555-555-5555", "examplePassword")
        .unwrap()
        .create(&test_connection);
    let organization = Organization::new(user.id).unwrap().create(&test_connection);
    let organization2 = Organization::new(user3.id)
        .unwrap()
        .create(&test_connection);
    OrganizationUser::new(organization2.id, user2.id)
        .unwrap()
        .create(&test_connection);

    // Owner is included in the user results for organization2 but not organization2
    let user_results = organization.users(&test_connection);
    assert!(user_results.len() == 1);
    assert_eq!(user.id, user_results[0].id);
    let user_results2 = organization2.users(&test_connection);
    assert!(user_results2.len() == 2);
    assert_eq!(user3.id, user_results2[0].id);
    assert_eq!(user2.id, user_results2[1].id);

    // Explicitly make the organization user an org user
    OrganizationUser::new(organization.id, user.id)
        .unwrap()
        .create(&test_connection);
    let user_results = organization.users(&test_connection);
    assert!(user_results.len() == 1);
    assert_eq!(user.id, user_results[0].id);
    let user_results2 = organization2.users(&test_connection);
    assert!(user_results2.len() == 2);
    assert_eq!(user3.id, user_results2[0].id);
    assert_eq!(user2.id, user_results2[1].id);

    // Add a new user to the organization
    OrganizationUser::new(organization.id, user2.id)
        .unwrap()
        .create(&test_connection);
    let user_results = organization.users(&test_connection);
    assert!(user_results.len() == 2);
    assert_eq!(user.id, user_results[0].id);
    assert_eq!(user2.id, user_results[1].id);
    let user_results2 = organization2.users(&test_connection);
    assert!(user_results2.len() == 2);
    assert_eq!(user3.id, user_results2[0].id);
    assert_eq!(user2.id, user_results2[1].id);
}
