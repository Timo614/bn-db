use bigneon_db::models::{Artist, Event, EventArtist, Organization, User, Venue};
use support::database;

#[test]
fn create_succeeds() {
    let test_connection = database::establish_test_connection();
    let artist = Artist::new("Name").unwrap().create(&test_connection);
    let venue = Venue::new("Name").unwrap().create(&test_connection);
    let user = User::new("Jeff", "jeff@tari.com", "555-555-5555", "examplePassword")
        .unwrap()
        .create(&test_connection);
    let organization = Organization::new(user.id).unwrap().create(&test_connection);
    let event = Event::new(organization.id, venue.id)
        .unwrap()
        .create(&test_connection);
    let rank = 1;

    let event_artist = EventArtist::new(event.id, artist.id, rank)
        .unwrap()
        .create(&test_connection);

    assert_eq!(event_artist.event_id, event.id);
    assert_eq!(event_artist.artist_id, artist.id);
    assert_eq!(event_artist.rank, rank);
    assert_eq!(event_artist.id.to_string().is_empty(), false);
}
