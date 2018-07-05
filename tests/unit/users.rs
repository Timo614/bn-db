use bigneon_db::models::User;
use support::project::TestProject;

#[test]
fn create() {
    let project = TestProject::new();
    let name = "Jeff";
    let email = "jeff@tari.com";
    let phone_number = "555-555-5555";
    let password = "examplePassword";
    let user = User::create(name, email, phone_number, password)
        .commit(&project)
        .unwrap();

    assert_eq!(user.name, name);
    assert_eq!(user.email, email);
    assert_eq!(user.phone, phone_number);
    assert_ne!(user.hashed_pw, password);
    assert_eq!(user.hashed_pw.is_empty(), false);
    assert_eq!(user.id.to_string().is_empty(), false);
}
