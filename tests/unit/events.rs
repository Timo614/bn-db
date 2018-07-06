use bigneon_db::models::{Event, Organization, User, Venue};
use support::project::TestProject;

#[test]
fn create() {
    let project = TestProject::new();
    let venue = Venue::create("Name").commit(&project).unwrap();
    let user = User::create("Jeff", "jeff@tari.com", "555-555-5555", "examplePassword")
        .commit(&project)
        .unwrap();
    let organization = Organization::create(user.id).commit(&project).unwrap();
    let event = Event::create(organization.id, venue.id)
        .commit(&project)
        .unwrap();
    assert_eq!(event.venue_id, venue.id);
    assert_eq!(event.organization_id, organization.id);
    assert_eq!(event.id.to_string().is_empty(), false);
}
