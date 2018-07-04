use bigneon_db::models::{Organization, OrganizationVenue, User, Venue};
use support::project::TestProject;

#[test]
fn create_succeeds() {
    let project = TestProject::new();
    let user = User::create("Jeff", "jeff@tari.com", "555-555-5555", "examplePassword")
        .commit(&project)
        .unwrap();
    let venue = Venue::create("Name").commit(&project).unwrap();
    let organization = Organization::create(user.id).commit(&project).unwrap();
    let organization_venue = OrganizationVenue::create(organization.id, venue.id)
        .commit(&project)
        .unwrap();

    assert_eq!(organization_venue.venue_id, venue.id);
    assert_eq!(organization_venue.organization_id, organization.id);
    assert_eq!(organization_venue.id.to_string().is_empty(), false);
}
