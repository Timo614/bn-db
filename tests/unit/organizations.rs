use bigneon_db::models::{Organization, OrganizationUser, User};
use support::project::TestProject;

#[test]
fn create() {
    let project = TestProject::new();
    let user = User::create("Jeff", "jeff@tari.com", "555-555-5555", "examplePassword")
        .commit(&project)
        .unwrap();
    let organization = Organization::create(user.id).commit(&project).unwrap();

    assert_eq!(organization.owner_user_id, user.id);
    assert_eq!(organization.id.to_string().is_empty(), false);
}

#[test]
fn users() {
    let project = TestProject::new();
    let user = User::create("Jeff", "jeff@tari.com", "555-555-5555", "examplePassword")
        .commit(&project)
        .unwrap();
    let user2 = User::create("David", "david@tari.com", "555-555-5555", "examplePassword")
        .commit(&project)
        .unwrap();
    let user3 = User::create("Ann", "ann@tari.com", "555-555-5555", "examplePassword")
        .commit(&project)
        .unwrap();
    let organization = Organization::create(user.id).commit(&project).unwrap();
    let organization2 = Organization::create(user3.id).commit(&project).unwrap();
    OrganizationUser::create(organization2.id, user2.id)
        .commit(&project)
        .unwrap();

    // Owner is included in the user results for organization2 but not organization2
    let user_results = organization.users(&project);
    assert!(user_results.len() == 1);
    assert_eq!(user.id, user_results[0].id);
    let user_results2 = organization2.users(&project);
    assert!(user_results2.len() == 2);
    assert_eq!(user3.id, user_results2[0].id);
    assert_eq!(user2.id, user_results2[1].id);

    // Explicitly make the organization user an org user
    OrganizationUser::create(organization.id, user.id)
        .commit(&project)
        .unwrap();
    let user_results = organization.users(&project);
    assert!(user_results.len() == 1);
    assert_eq!(user.id, user_results[0].id);
    let user_results2 = organization2.users(&project);
    assert!(user_results2.len() == 2);
    assert_eq!(user3.id, user_results2[0].id);
    assert_eq!(user2.id, user_results2[1].id);

    // Add a new user to the organization
    OrganizationUser::create(organization.id, user2.id)
        .commit(&project)
        .unwrap();
    let user_results = organization.users(&project);
    assert!(user_results.len() == 2);
    assert_eq!(user.id, user_results[0].id);
    assert_eq!(user2.id, user_results[1].id);
    let user_results2 = organization2.users(&project);
    assert!(user_results2.len() == 2);
    assert_eq!(user3.id, user_results2[0].id);
    assert_eq!(user2.id, user_results2[1].id);
}
