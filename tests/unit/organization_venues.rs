use bigneon_db::models::{Organization, OrganizationVenue, User, Venue};
use support::database;

#[test]
fn create_succeeds() {
    let test_connection = database::establish_test_connection();
    let user = User::new("Jeff", "jeff@tari.com", "555-555-5555", "examplePassword")
        .unwrap()
        .create(&test_connection);
    let venue = Venue::new("Name").unwrap().create(&test_connection);
    let organization = Organization::new(user.id).unwrap().create(&test_connection);
    let organization_venue = OrganizationVenue::new(organization.id, venue.id)
        .unwrap()
        .create(&test_connection);

    assert_eq!(organization_venue.venue_id, venue.id);
    assert_eq!(organization_venue.organization_id, organization.id);
    assert_eq!(organization_venue.id.to_string().is_empty(), false);
}
