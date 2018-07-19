use bigneon_db::models::{Organization, OrganizationVenue, User, Venue};
use support::project::TestProject;

#[test]
fn create() {
    let project = TestProject::new();
    let user = User::create("Jeff", "jeff@tari.com", "555-555-5555", "examplePassword")
        .commit(&project)
        .unwrap();
    let venue = Venue::create("Name").commit(&project).unwrap();
    let organization = Organization::create(user.id, "Organization")
        .commit(&project)
        .unwrap();
    let organization_venue = OrganizationVenue::create(organization.id, venue.id)
        .commit(&project)
        .unwrap();

    assert_eq!(organization_venue.venue_id, venue.id);
    assert_eq!(organization_venue.organization_id, organization.id);
    assert_eq!(organization_venue.id.to_string().is_empty(), false);
}

#[test]
fn find() {
    //create user and organization
    let project = TestProject::new();
    let user = User::create("Jeff", "jeff@tari.com", "555-555-5555", "examplePassword")
        .commit(&project)
        .unwrap();
    let mut edited_organization = Organization::create(user.id, "OrganizationForFindTest2")
        .commit(&project)
        .unwrap();
    edited_organization.address = Some(<String>::from("Test Address2"));
    edited_organization.city = Some(<String>::from("Test Address2"));
    edited_organization.state = Some(<String>::from("Test state2"));
    edited_organization.country = Some(<String>::from("Test country2"));
    edited_organization.zip = Some(<String>::from("0125"));
    edited_organization.phone = Some(<String>::from("+27123456780"));
    let updated_organization = Organization::update(&edited_organization, &project).unwrap();
    //create Venue
    let mut edited_venue = Venue::create("VenueForFindTest").commit(&project).unwrap();
    edited_venue.address = Some(<String>::from("Test Address"));
    edited_venue.city = Some(<String>::from("Test Address"));
    edited_venue.state = Some(<String>::from("Test state"));
    edited_venue.country = Some(<String>::from("Test country"));
    edited_venue.zip = Some(<String>::from("0124"));
    edited_venue.phone = Some(<String>::from("+27123456789"));
    let updated_venue = Venue::update(&edited_venue, &project).unwrap();
    //create organization>venue link
    let _organization_venue = OrganizationVenue::create(updated_organization.id, updated_venue.id)
        .commit(&project)
        .unwrap();
    //find organization linked to venue
    let organization_id =
        OrganizationVenue::find_via_venue_all(&updated_venue.id, &project).unwrap();
    let found_organization =
        Organization::find(&organization_id[0].organization_id, &project).unwrap();
    assert_eq!(found_organization, updated_organization);
    //find venue linked to organization
    let venue_id =
        OrganizationVenue::find_via_organization_all(&updated_organization.id, &project).unwrap();
    let found_venue = Venue::find(&venue_id[0].venue_id, &project).unwrap();
    assert_eq!(found_venue, updated_venue);
}
#[test]
fn find_lists() {
    //create user and organization
    let project = TestProject::new();
    let user = User::create("Jeff", "jeff@tari.com", "555-555-5555", "examplePassword")
        .commit(&project)
        .unwrap();
    let mut edited_organization = Organization::create(user.id, "OrganizationForFindTest2")
        .commit(&project)
        .unwrap();
    edited_organization.address = Some(<String>::from("Test Address2"));
    edited_organization.city = Some(<String>::from("Test Address2"));
    edited_organization.state = Some(<String>::from("Test state2"));
    edited_organization.country = Some(<String>::from("Test country2"));
    edited_organization.zip = Some(<String>::from("0125"));
    edited_organization.phone = Some(<String>::from("+27123456780"));
    let updated_organization = Organization::update(&edited_organization, &project).unwrap();
    let mut edited_organization2 = Organization::create(user.id, "OrganizationForFindTest2")
        .commit(&project)
        .unwrap();
    edited_organization2.address = Some(<String>::from("Test Address2"));
    edited_organization2.city = Some(<String>::from("Test Address2"));
    edited_organization2.state = Some(<String>::from("Test state2"));
    edited_organization2.country = Some(<String>::from("Test country2"));
    edited_organization2.zip = Some(<String>::from("0125"));
    edited_organization2.phone = Some(<String>::from("+27123456780"));
    let updated_organization2 = Organization::update(&edited_organization2, &project).unwrap();
    let all_organizations = vec![updated_organization, updated_organization2];

    //create Venue
    let mut edited_venue = Venue::create("VenueForFindTest").commit(&project).unwrap();
    edited_venue.address = Some(<String>::from("Test Address"));
    edited_venue.city = Some(<String>::from("Test Address"));
    edited_venue.state = Some(<String>::from("Test state"));
    edited_venue.country = Some(<String>::from("Test country"));
    edited_venue.zip = Some(<String>::from("0124"));
    edited_venue.phone = Some(<String>::from("+27123456789"));
    let updated_venue = Venue::update(&edited_venue, &project).unwrap();
    let mut edited_venue2 = Venue::create("VenueForFindTest").commit(&project).unwrap();
    edited_venue2.address = Some(<String>::from("Test Address"));
    edited_venue2.city = Some(<String>::from("Test Address"));
    edited_venue2.state = Some(<String>::from("Test state"));
    edited_venue2.country = Some(<String>::from("Test country"));
    edited_venue2.zip = Some(<String>::from("0124"));
    edited_venue2.phone = Some(<String>::from("+27123456789"));
    let updated_venue2 = Venue::update(&edited_venue2, &project).unwrap();
    let all_venues = vec![updated_venue, updated_venue2];

    //create organization > venue links
    let _organization_venue = OrganizationVenue::create(all_organizations[0].id, all_venues[0].id)
        .commit(&project)
        .unwrap();
    let _organization_venue = OrganizationVenue::create(all_organizations[0].id, all_venues[1].id)
        .commit(&project)
        .unwrap();
    let _organization_venue = OrganizationVenue::create(all_organizations[1].id, all_venues[0].id)
        .commit(&project)
        .unwrap();
    let _organization_venue = OrganizationVenue::create(all_organizations[1].id, all_venues[1].id)
        .commit(&project)
        .unwrap();
    //find organization linked to venue
    let organization_ids =
        OrganizationVenue::find_via_venue_all(&all_venues[0].id, &project).unwrap();
    let mut found_organizations: Vec<Organization> = Vec::new();
    for i in 0..=1 {
        found_organizations
            .push(Organization::find(&organization_ids[i].organization_id, &project).unwrap());
    }
    assert_eq!(found_organizations, all_organizations);
    //find venue linked to organization
    let venue_ids =
        OrganizationVenue::find_via_organization_all(&all_organizations[0].id, &project).unwrap();
    let mut found_venue: Vec<Venue> = Vec::new();
    for i in 0..=1 {
        found_venue.push(Venue::find(&venue_ids[i].venue_id, &project).unwrap());
    }
    assert_eq!(found_venue, all_venues);
}
