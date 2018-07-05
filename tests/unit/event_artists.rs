use bigneon_db::models::{Artist, Event, EventArtist, Organization, User, Venue};
use support::project::TestProject;

#[test]
fn create() {
    let project = TestProject::new();
    let artist = Artist::create("Name").commit(&project).unwrap();
    let venue = Venue::create("Name").commit(&project).unwrap();
    let user = User::create("Jeff", "jeff@tari.com", "555-555-5555", "examplePassword")
        .commit(&project)
        .unwrap();
    let organization = Organization::create(user.id).commit(&project).unwrap();
    let event = Event::create(organization.id, venue.id)
        .commit(&project)
        .unwrap();
    let rank = 1;

    let event_artist = EventArtist::new(event.id, artist.id, rank)
        .unwrap()
        .create(&project);

    assert_eq!(
        event_artist.event_id, event.id,
        "Event foreign key does not match"
    );
    assert_eq!(
        event_artist.artist_id, artist.id,
        "Artist foreign key does not match"
    );
    assert_eq!(event_artist.rank, rank, "Artist rank is not correct");
    assert_eq!(event_artist.id.to_string().is_empty(), false);
}
