use bigneon_db::models::{Organization, OrganizationUser};
use support::project::TestProject;
use uuid::Uuid;

#[test]
fn create() {
    let project = TestProject::new();
    let user = project.create_user().finish();
    let organization = Organization::create(user.id, "Organization")
        .commit(&project)
        .unwrap();

    assert_eq!(organization.owner_user_id, user.id);
    assert_eq!(organization.id.to_string().is_empty(), false);
}

#[test]
fn update() {
    let project = TestProject::new();
    let user = project.create_user().finish();
    //Edit Organization
    let mut edited_organization = project.create_organization().with_owner(&user).finish();

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
    let user = project.create_user().finish();
    //Edit Organization
    let mut edited_organization = project.create_organization().with_owner(&user).finish();

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
}

#[test]
fn users() {
    let project = TestProject::new();
    let user = project.create_user().finish();
    let user2 = project.create_user().finish();
    let user3 = project.create_user().finish();
    let organization = project.create_organization().with_owner(&user).finish();
    let organization2 = project.create_organization().with_owner(&user3).finish();
    OrganizationUser::create(organization2.id, user2.id)
        .commit(&project)
        .unwrap();

    // Owner is included in the user results for organization2 but not organization2
    let user_results = organization.users(&project);
    assert_eq!(user_results.len(), 1);
    assert_eq!(user.id, user_results[0].id);

    let user_results = organization2.users(&project);
    assert_eq!(
        vec![user3.id, user2.id],
        user_results.iter().map(|u| u.id).collect::<Vec<Uuid>>()
    );

    // Explicitly make the organization user an org user
    OrganizationUser::create(organization.id, user.id)
        .commit(&project)
        .unwrap();
    let user_results = organization.users(&project);
    assert_eq!(user_results.len(), 1);
    assert_eq!(user.id, user_results[0].id);
    let user_results2 = organization2.users(&project);
    assert_eq!(user_results2.len(), 2);
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

#[test]
fn all() {
    let project = TestProject::new();
    let user = project.create_user().finish();
    let user2 = project.create_user().finish();
    let org1 = project.create_organization().with_owner(&user).finish();
    let org2 = project
        .create_organization()
        .with_owner(&user2)
        .with_user(&user)
        .finish();
    let _org3 = project.create_organization().with_owner(&user2).finish();

    let orgs = Organization::all(user.id, &project).unwrap();

    assert_eq!(orgs.len(), 2);
    assert_eq!(orgs[0].id, org1.id);
    assert_eq!(orgs[1].id, org2.id);
}
