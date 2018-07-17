use bigneon_db::models::User;
use support::project::TestProject;

#[test]
fn commit() {
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

#[test]
fn find_by_email() {
    let project = TestProject::new();
    let email = "jeff@tari.com";
    User::create("Jeff", email, "555-555-5555", "examplePassword")
        .commit(&project)
        .unwrap();

    let found_user = match User::find_by_email(email, &project) {
        Ok(_user) => true,
        Err(_e) => false,
    };
    assert!(found_user, "User not found");

    let invalid_user = match User::find_by_email("not@real.com", &project) {
        Ok(_user) => false,
        Err(_e) => true,
    };
    assert!(invalid_user, "User incorrectly returned when email invalid");
}
