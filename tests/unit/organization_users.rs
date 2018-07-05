use bigneon_db::models::{Organization, OrganizationUser, User};
use support::project::TestProject;

#[test]
fn create() {
    let project = TestProject::new();
    let user = User::create("Jeff", "jeff@tari.com", "555-555-5555", "examplePassword")
        .commit(&project)
        .unwrap();
    let user2 = User::create("Dan", "dan@tari.com", "555-555-5555", "examplePassword")
        .commit(&project)
        .unwrap();
    let organization = Organization::create(user.id).commit(&project).unwrap();
    let organization_user = OrganizationUser::create(organization.id, user2.id)
        .commit(&project)
        .unwrap();

    assert_eq!(organization_user.user_id, user2.id);
    assert_eq!(organization_user.organization_id, organization.id);
    assert_eq!(organization_user.id.to_string().is_empty(), false);
}
