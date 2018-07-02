use bigneon_db::models::User;
use support::database;

#[test]
fn create_succeeds() {
    let test_connection = database::establish_test_connection();
    let name = "Jeff";
    let email = "jeff@tari.com";
    let phone_number = "555-555-5555";
    let password = "examplePassword";
    let user = User::new(name, email, phone_number, password)
        .unwrap()
        .create(&test_connection);

    assert_eq!(user.name, name);
    assert_eq!(user.email, email);
    assert_eq!(user.phone, phone_number);
    assert_ne!(user.hashed_pw, password);
    assert_eq!(user.hashed_pw.is_empty(), false);
    assert_eq!(user.id.to_string().is_empty(), false);
}
