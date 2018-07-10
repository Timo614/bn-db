use bigneon_db::models::{Organization, OrganizationUser, User};
use support::project::TestProject;

#[test]
fn create() {
    let project = TestProject::new();
    let user = User::create("Jeff", "jeff@tari.com", "555-555-5555", "examplePassword")
        .commit(&project)
        .unwrap();
    let organization = Organization::create(user.id, "Organization")
        .commit(&project)
        .unwrap();

    assert_eq!(organization.owner_user_id, user.id);
    assert_eq!(organization.id.to_string().is_empty(), false);
}

#[test]
fn update() {
    let project = TestProject::new();
    let user = User::create("Jeff", "jeff@tari.com", "555-555-5555", "examplePassword")
        .commit(&project)
        .unwrap();
    //Edit Organization
    let mut edited_organization = Organization::create(user.id, "Organization")
        .commit(&project)
        .unwrap();
    edited_organization.address = Some(<String>::from("Test Address"));
    edited_organization.city = Some(<String>::from("Test Address"));
    edited_organization.state = Some(<String>::from("Test state"));
    edited_organization.country = Some(<String>::from("Test country"));
    edited_organization.zip = Some(<String>::from("0124"));
    edited_organization.phone = Some(<String>::from("+27123456789"));
    let updated_organization = Organization::update(&edited_organization, &project).unwrap();
    assert_eq!(edited_organization, updated_organization);
}

#[test]
fn find() {
    let project = TestProject::new();
    let user = User::create("Jeff", "jeff@tari.com", "555-555-5555", "examplePassword")
        .commit(&project)
        .unwrap();
    //Edit Organization
    let mut edited_organization = Organization::create(user.id, "OrganizationForFindTest")
        .commit(&project)
        .unwrap();
    edited_organization.address = Some(<String>::from("Test Address"));
    edited_organization.city = Some(<String>::from("Test Address"));
    edited_organization.state = Some(<String>::from("Test state"));
    edited_organization.country = Some(<String>::from("Test country"));
    edited_organization.zip = Some(<String>::from("0124"));
    edited_organization.phone = Some(<String>::from("+27123456789"));
    //find organization
    let _updated_organization = Organization::update(&edited_organization, &project).unwrap();
    let found_organization = Organization::find(&edited_organization.id, &project).unwrap();
    assert_eq!(edited_organization, found_organization);

    //find more than one organization
    let mut edited_organization2 = Organization::create(user.id, "OrganizationForFindTest2")
        .commit(&project)
        .unwrap();
    edited_organization2.address = Some(<String>::from("Test Address2"));
    edited_organization2.city = Some(<String>::from("Test Address2"));
    edited_organization2.state = Some(<String>::from("Test state2"));
    edited_organization2.country = Some(<String>::from("Test country2"));
    edited_organization2.zip = Some(<String>::from("0125"));
    edited_organization2.phone = Some(<String>::from("+27123456780"));
    let _updated_organization = Organization::update(&edited_organization2, &project).unwrap();
    let all_found_organizations = Organization::all(&project).unwrap();
    let all_organizations = vec![edited_organization, edited_organization2];
    assert_eq!(all_organizations, all_found_organizations);
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
    let organization = Organization::create(user.id, "Organization")
        .commit(&project)
        .unwrap();
    let organization2 = Organization::create(user3.id, "Organization")
        .commit(&project)
        .unwrap();
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
