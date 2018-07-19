extern crate chrono;
use bigneon_db::models::{Event, Order, Organization, User, Venue};
use support::project::TestProject;
use unit::orders::chrono::prelude::*;

#[test]
fn create() {
    let project = TestProject::new();
    let venue = Venue::create("Name").commit(&project).unwrap();
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
    let order = Order::create(user.id, event.id).commit(&project).unwrap();
    assert_eq!(order.event_id, event.id);
    assert_eq!(order.user_id, user.id);
    assert_eq!(order.id.to_string().is_empty(), false);
}
