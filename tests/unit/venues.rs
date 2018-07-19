use bigneon_db::models::{Organization, User, Venue};
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
    let _updated_venue = Venue::update(&edited_venue2, &project).unwrap();
    let all_found_venues = Venue::all(&project).unwrap();
    let all_venues = vec![edited_venue, edited_venue2];
    assert_eq!(all_venues, all_found_venues);
}

#[test]
fn create_find_via_org() {
    let project = TestProject::new();
    //create Venues
    let mut edited_venue = Venue::create("VenueForOrgTest").commit(&project).unwrap();
    edited_venue.address = Some(<String>::from("Test Address"));
    edited_venue.city = Some(<String>::from("Test Address"));
    edited_venue.state = Some(<String>::from("Test state"));
    edited_venue.country = Some(<String>::from("Test country"));
    edited_venue.zip = Some(<String>::from("0124"));
    edited_venue.phone = Some(<String>::from("+27123456789"));
    let updated_venue = Venue::update(&edited_venue, &project).unwrap();
    let mut edited_venue2 = Venue::create("VenueForOrgTest").commit(&project).unwrap();
    edited_venue2.address = Some(<String>::from("Test Address"));
    edited_venue2.city = Some(<String>::from("Test Address"));
    edited_venue2.state = Some(<String>::from("Test state"));
    edited_venue2.country = Some(<String>::from("Test country"));
    edited_venue2.zip = Some(<String>::from("0124"));
    edited_venue2.phone = Some(<String>::from("+27123456789"));
    let updated_venue2 = Venue::update(&edited_venue2, &project).unwrap();
    //create user
    let user = User::create("Jeff", "jeff@tari.com", "555-555-5555", "examplePassword")
        .commit(&project)
        .unwrap();
    //create organization
    let mut edited_organization = Organization::create(user.id, "OrganizationforVenue")
        .commit(&project)
        .unwrap();
    edited_organization.address = Some(<String>::from("Test Address"));
    edited_organization.city = Some(<String>::from("Test Address"));
    edited_organization.state = Some(<String>::from("Test state"));
    edited_organization.country = Some(<String>::from("Test country"));
    edited_organization.zip = Some(<String>::from("0124"));
    edited_organization.phone = Some(<String>::from("+27123456789"));
    let updated_organization = Organization::update(&edited_organization, &project).unwrap();
    //Do linking
    let _org_venue_link = updated_venue.add_to_organization(&updated_organization.id, &project);
    let _org_venue_link = updated_venue2.add_to_organization(&updated_organization.id, &project);
    let all_venues = vec![updated_venue, updated_venue2];
    let found_venues =
        Venue::find_all_for_organization(&updated_organization.id, &project).unwrap();
    assert_eq!(found_venues, all_venues);
}
