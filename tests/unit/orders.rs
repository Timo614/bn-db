use bigneon_db::models::{Event, Order, Organization, User, Venue};
use support::database;

#[test]
fn create_succeeds() {
    let test_connection = database::establish_test_connection();
    let venue = Venue::new("Name").unwrap().create(&test_connection);
    let user = User::new("Jeff", "jeff@tari.com", "555-555-5555", "examplePassword")
        .unwrap()
        .create(&test_connection);
    let organization = Organization::new(user.id).unwrap().create(&test_connection);
    let event = Event::new(organization.id, venue.id)
        .unwrap()
        .create(&test_connection);
    let order = Order::new(user.id, event.id)
        .unwrap()
        .create(&test_connection);
    assert_eq!(order.event_id, event.id);
    assert_eq!(order.user_id, user.id);
    assert_eq!(order.id.to_string().is_empty(), false);
}
