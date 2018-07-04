use bigneon_db::models::Artist;
use support::project::TestProject;

#[test]
fn create_succeeds() {
    let project = TestProject::new();
    let name = "Name";
    let artist = Artist::create(&name).commit(&project).unwrap();
    assert_eq!(name, artist.name);
    assert_eq!(artist.id.to_string().is_empty(), false);
}
