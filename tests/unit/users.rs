use bigneon_db::db::connections::Connectable;
use bigneon_db::models::Roles;
use bigneon_db::models::User;
use bigneon_db::models::users::Resetable;
use chrono::{Duration, Utc};
use diesel;
use diesel::prelude::*;
use support::project::TestProject;
use uuid::Uuid;

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
fn find() {
    let project = TestProject::new();
    let user = User::create("Jeff", "jeff@tari.com", "555-555-5555", "examplePassword")
        .commit(&project)
        .unwrap();

    let found_user = User::find(&user.id, &project).expect("User was not found");
    assert_eq!(found_user.id, user.id);
    assert_eq!(found_user.email, user.email);

    assert!(
        match User::find(&Uuid::new_v4(), &project) {
            Ok(_user) => false,
            Err(_e) => true,
        },
        "User incorrectly returned when id invalid"
    );
}

#[test]
fn find_by_email() {
    let project = TestProject::new();
    let email = "jeff@tari.com";
    let user = User::create("Jeff", email, "555-555-5555", "examplePassword")
        .commit(&project)
        .unwrap();

    let found_user = User::find_by_email(&email, &project).expect("User was not found");
    assert_eq!(found_user.id, user.id);
    assert_eq!(found_user.email, user.email);

    assert!(
        match User::find_by_email("not@real.com", &project) {
            Ok(_user) => false,
            Err(_e) => true,
        },
        "User incorrectly returned when email invalid"
    );
}

#[test]
fn find_by_password_reset_token() {
    let project = TestProject::new();
    let user = User::create("Jeff", "jeff@tari.com", "555-555-5555", "examplePassword")
        .commit(&project)
        .unwrap();
    let token : Resetable = user.create_password_reset_token(&project).expect("Failed to create reset token").into();

    let found_user = User::find_by_password_reset_token(&token.password_reset_token.unwrap(), &project).expect("User was not found");
    assert_eq!(found_user.id, user.id);
    assert_eq!(found_user.password_reset_token.unwrap(), token.password_reset_token.unwrap());

    assert!(
        match User::find_by_password_reset_token(&Uuid::new_v4(), &project) {
            Ok(_user) => false,
            Err(_e) => true,
        },
        "User incorrectly returned when password token invalid"
    );
}

#[test]
fn consume_password_reset_token() {
    use bigneon_db::schema::users::dsl::*;
    let project = TestProject::new();
    let user = User::create("Jeff", "jeff@tari.com", "555-555-5555", "examplePassword")
        .commit(&project)
        .unwrap();
    let token : Resetable = user.create_password_reset_token(&project).expect("Failed to create reset token").into();
    let password = "newPassword";
    assert!(!user.check_password(&password));

    // Consumes password reset as token was not expired and valid
    let user = User::consume_password_reset_token(&token.password_reset_token.unwrap(), &password, &project).unwrap();
    assert!(user.check_password(&password));
    assert!(user.password_reset_token.is_none());
    assert!(user.password_reset_requested_at.is_none());

    // Does not consume password reset as token was expired although valid
    let user : User = diesel::update(users.filter(id.eq(user.id)))
        .set(
            Resetable {
                password_reset_token: Some(Uuid::new_v4()),
                password_reset_requested_at: Some(Utc::now().naive_utc() - Duration::days(3)),
            }
        )
        .get_result(project.get_connection()).unwrap();
    let password = "newPassword2";
    match User::consume_password_reset_token(&user.password_reset_token.unwrap(), &password, &project) {
        Ok(_v) => panic!("Expected failure to consume expired password reset token"),
        Err(e) => assert_eq!(format!("{}", e), "[5000] Internal error\nCaused by: Password reset token is expired"),
    }
    assert!(!user.check_password(&password));

    // Invalid token
    match User::consume_password_reset_token(&Uuid::new_v4(), &password, &project) {
        Ok(_v) => panic!("Expected failure to consume expired password reset token"),
        Err(e) => assert_eq!(format!("{}", e), "[3000] Query Error\nCaused by: Error loading user, NotFound"),
    }
    assert!(!user.check_password(&password));
}

#[test]
fn for_display() {
    let project = TestProject::new();
    let user = User::create("Jeff", "jeff@tari.com", "555-555-5555", "examplePassword")
        .commit(&project)
        .unwrap();
    let user_id = user.id.clone();
    let display_user = user.for_display();

    assert_eq!(display_user.id, user_id);
}

#[test]
fn create_password_reset_token() {
    let project = TestProject::new();
    let user = User::create("Jeff", "jeff@tari.com", "555-555-5555", "examplePassword")
        .commit(&project)
        .unwrap();
    assert!(user.password_reset_token.is_none());
    assert!(user.password_reset_requested_at.is_none());

    let user = user.create_password_reset_token(&project).unwrap();
    assert!(user.password_reset_token.is_some());
    assert!(user.password_reset_requested_at.is_some());
}

#[test]
fn add_role() {
    let project = TestProject::new();
    let user = User::create("Jeff", "jeff@tari.com", "555-555-5555", "examplePassword")
        .commit(&project)
        .unwrap();

    user.add_role(Roles::Admin, &project).unwrap();

    let user2 = User::find(&user.id, &project).unwrap();
    assert_eq!(user2.role, vec!["Guest", "Admin"]);
}

#[test]
fn is_expired() {
    // Expired token
    let token = Resetable {
        password_reset_token: Some(Uuid::new_v4()),
        password_reset_requested_at: Some(Utc::now().naive_utc() - Duration::days(2) - Duration::seconds(10)),
    };
    assert!(token.is_expired(), "Token should be expired");

    // Token not yet expired
    let token = Resetable {
        password_reset_token: Some(Uuid::new_v4()),
        password_reset_requested_at: Some(Utc::now().naive_utc() - Duration::days(2) + Duration::seconds(10)),
    };
    assert!(!token.is_expired(), "Token should not be expired");
}
