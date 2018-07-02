use bigneon_db::models::Venue;
use support::database;

#[test]
fn create_succeeds() {
    let test_connection = database::establish_test_connection();
    let venue = Venue::new("Name").unwrap().create(&test_connection);

    assert_eq!(venue.name, venue.name);
    assert_eq!(venue.id.to_string().is_empty(), false);
}
