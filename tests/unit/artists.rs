use bigneon_db::models::Artist;
use support::database;

#[test]
fn create_succeeds() {
    let test_connection = database::establish_test_connection();
    let name = "Name";
    let artist = Artist::new(&name).unwrap().create(&test_connection);
    assert_eq!(name, artist.name);
    assert_eq!(artist.id.to_string().is_empty(), false);
}
