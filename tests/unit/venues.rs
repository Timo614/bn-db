use bigneon_db::models::Venue;
use support::project::TestProject;

#[test]
fn create() {
    let project = TestProject::new();
    let venue = Venue::create("Name").commit(&project).unwrap();

    assert_eq!(venue.name, venue.name);
    assert_eq!(venue.id.to_string().is_empty(), false);
}

#[test]
fn update() {
    let project = TestProject::new();
    //Edit Venue
    let mut venue = Venue::create("NewVenue").commit(&project).unwrap();
    venue.address = Some(<String>::from("Test Address"));
    venue.city = Some(<String>::from("Test Address"));
    venue.state = Some(<String>::from("Test state"));
    venue.country = Some(<String>::from("Test country"));
    venue.zip = Some(<String>::from("0124"));
    venue.phone = Some(<String>::from("+27123456789"));
    let updated_venue = Venue::update(&venue, &project).unwrap();
    assert_eq!(venue, updated_venue);
}

#[test]
fn find() {
    let project = TestProject::new();
    //create Venue
    let mut edited_venue = Venue::create("VenueForFindTest").commit(&project).unwrap();
    edited_venue.address = Some(<String>::from("Test Address"));
    edited_venue.city = Some(<String>::from("Test Address"));
    edited_venue.state = Some(<String>::from("Test state"));
    edited_venue.country = Some(<String>::from("Test country"));
    edited_venue.zip = Some(<String>::from("0124"));
    edited_venue.phone = Some(<String>::from("+27123456789"));
    //find venue
    let _updated_organization = Venue::update(&edited_venue, &project).unwrap();
    let found_organization = Venue::find(&edited_venue.id, &project).unwrap();
    assert_eq!(edited_venue, found_organization);

    //find more than one venue
    let mut edited_venue2 = Venue::create("VenueForFindTest2").commit(&project).unwrap();
    edited_venue2.address = Some(<String>::from("Test Address2"));
    edited_venue2.city = Some(<String>::from("Test Address2"));
    edited_venue2.state = Some(<String>::from("Test state2"));
    edited_venue2.country = Some(<String>::from("Test country2"));
    edited_venue2.zip = Some(<String>::from("0125"));
    edited_venue2.phone = Some(<String>::from("+27123456780"));
    let _updated_organization = Venue::update(&edited_venue2, &project).unwrap();
    let all_found_venues = Venue::all(&project).unwrap();
    let all_venues = vec![edited_venue, edited_venue2];
    assert_eq!(all_venues, all_found_venues);
}
