extern crate chrono;
use bigneon_db::models::{Event, Organization, User, Venue};
use support::project::TestProject;
use unit::events::chrono::prelude::*;

#[test]
fn create() {
    let project = TestProject::new();
    let venue = Venue::create("Venue").commit(&project).unwrap();
    let user = User::create("Jeff", "jeff@tari.com", "555-555-5555", "examplePassword")
        .commit(&project)
        .unwrap();
    let organization = Organization::create(user.id, "Organization")
        .commit(&project)
        .unwrap();
    let event = Event::create(
        organization.id,
        venue.id,
        NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
    ).commit(&project)
        .unwrap();
    assert_eq!(event.venue_id, venue.id);
    assert_eq!(event.organization_id, organization.id);
    assert_eq!(event.id.to_string().is_empty(), false);
}
#[test]
fn update() {
    let _project = TestProject::new();
    //create event
    let project = TestProject::new();
    let venue = Venue::create("Venue").commit(&project).unwrap();

    let user = User::create("Jeff", "jeff@tari.com", "555-555-5555", "examplePassword")
        .commit(&project)
        .unwrap();
    let organization = Organization::create(user.id, "Organization")
        .commit(&project)
        .unwrap();
    let mut event = Event::create(
        organization.id,
        venue.id,
        NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
    ).commit(&project)
        .unwrap();
    //Edit event
    event.ticket_sell_date = NaiveDate::from_ymd(2017, 7, 8).and_hms(9, 10, 11);
    let updated_event = Event::update(&event, &project).unwrap();
    assert_eq!(event, updated_event);
}

#[test]
fn find_individuals() {
    let _project = TestProject::new();
    //create event
    let project = TestProject::new();
    let venue = Venue::create("Venue").commit(&project).unwrap();

    let user = User::create("Jeff", "jeff@tari.com", "555-555-5555", "examplePassword")
        .commit(&project)
        .unwrap();
    let organization = Organization::create(user.id, "Organization")
        .commit(&project)
        .unwrap();
    let mut event = Event::create(
        organization.id,
        venue.id,
        NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
    ).commit(&project)
        .unwrap();
    //Edit event
    event.ticket_sell_date = NaiveDate::from_ymd(2017, 7, 8).and_hms(9, 10, 11);
    let updated_event = Event::update(&event, &project).unwrap();

    //find event
    let found_event = Event::find(&updated_event.id, &project).unwrap();
    assert_eq!(found_event, updated_event);
    //find event via organisation
    let found_event_via_organization =
        Event::find_all_events_from_organization(&found_event.organization_id, &project).unwrap();
    assert_eq!(found_event_via_organization[0], found_event);

    //find event via venue
    let found_event_via_venue =
        Event::find_all_events_from_venue(&updated_event.venue_id, &project).unwrap();
    assert_eq!(found_event_via_venue[0], updated_event);
}

#[test]
fn find_list() {
    let _project = TestProject::new();
    //create event
    let project = TestProject::new();
    let venue = Venue::create("VenueL").commit(&project).unwrap();
    let user = User::create("Jeff", "jeff@tari.com", "555-555-5555", "examplePassword")
        .commit(&project)
        .unwrap();
    let organization = Organization::create(user.id, "OrganizationL")
        .commit(&project)
        .unwrap();
    let mut event = Event::create(
        organization.id,
        venue.id,
        NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
    ).commit(&project)
        .unwrap();
    //Edit event
    event.ticket_sell_date = NaiveDate::from_ymd(2017, 7, 8).and_hms(9, 10, 11);
    let updated_event = Event::update(&event, &project).unwrap();

    //find more than one event
    let mut event2 = Event::create(
        organization.id,
        venue.id,
        NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
    ).commit(&project)
        .unwrap();
    event2.ticket_sell_date = NaiveDate::from_ymd(2018, 7, 8).and_hms(9, 10, 11);
    let _updated_event2 = Event::update(&event2, &project).unwrap();
    let all_found_events = Event::all(&project).unwrap();
    let all_events = vec![event, event2];
    assert_eq!(all_events, all_found_events);

    //find all events via organisation
    let found_event_via_organizations =
        Event::find_all_events_from_organization(&updated_event.organization_id, &project).unwrap();
    assert_eq!(found_event_via_organizations, all_events);

    //find all events via venue
    let found_event_via_venues =
        Event::find_all_events_from_venue(&updated_event.venue_id, &project).unwrap();
    assert_eq!(found_event_via_venues, all_events);
}
