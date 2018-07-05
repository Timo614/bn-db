use bigneon_db::models::Venue;
use support::project::TestProject;

#[test]
fn create() {
    let project = TestProject::new();
    let venue = Venue::create("Name").commit(&project).unwrap();

    assert_eq!(venue.name, venue.name);
    assert_eq!(venue.id.to_string().is_empty(), false);
}
