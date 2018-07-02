use bigneon_db::models::{Event, EventHistory, Order, Organization, User, Venue};
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
    let protocol_reference_hash = "HASH";
    let event_history = EventHistory::new(event.id, order.id, user.id, protocol_reference_hash)
        .unwrap()
        .create(&test_connection);
    assert_eq!(event_history.event_id, event.id);
    assert_eq!(event_history.order_id, order.id);
    assert_eq!(event_history.user_id, user.id);
    assert_eq!(
        event_history.protocol_reference_hash,
        protocol_reference_hash
    );
    assert_eq!(event_history.id.to_string().is_empty(), false);
}
