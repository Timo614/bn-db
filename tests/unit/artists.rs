use bigneon_db::models::{Artist, NewArtist};
use support::project::TestProject;

#[test]
fn commit() {
    let project = TestProject::new();
    let name = "Name";
    let artist = Artist::create(&name).commit(&project).unwrap();
    assert_eq!(name, artist.name);
    assert_eq!(artist.id.to_string().is_empty(), false);
}

#[test]
fn find() {
    let project = TestProject::new();
    let name = "Name";
    let artist = Artist::create(&name).commit(&project).unwrap();
    assert_eq!(name, artist.name);
    assert_eq!(artist.id.to_string().is_empty(), false);

    let found_artist = Artist::find(&artist.id, &project);
    assert_eq!(found_artist.id, artist.id);
    assert_eq!(found_artist.name, artist.name);
}

#[test]
fn all() {
    let project = TestProject::new();
    let name = "Name";
    let artist = Artist::create(&name).commit(&project).unwrap();
    assert_eq!(name, artist.name);
    assert_eq!(artist.id.to_string().is_empty(), false);

    let found_artists = Artist::all(&project);
    assert_eq!(1, found_artists.len());
    assert_eq!(found_artists[0].id, artist.id);
    assert_eq!(found_artists[0].name, artist.name);
}

#[test]
fn update_attributes(){
    let project = TestProject::new();
    let name = "Old Name";
    let artist = Artist::create(&name).commit(&project).unwrap();

    let artist_parameters = NewArtist{name: "New Name".to_string()};
    let updated_artist = artist.update_attributes(&artist_parameters, &project);

    assert_eq!(updated_artist.id, artist.id);
    assert_ne!(updated_artist.name, artist.name);
    assert_eq!(updated_artist.name, artist_parameters.name);
}

#[test]
#[should_panic]
fn destroy(){
    let project = TestProject::new();
    let name = "Old Name";
    let artist = Artist::create(&name).commit(&project).unwrap();
    assert!(artist.destroy(&project));

    // Force panic proving record no longer exists
    Artist::find(&artist.id, &project);
}
