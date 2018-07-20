use bigneon_db::db::connections::Connectable;
use bigneon_db::models::User;
use bigneon_db::models::concerns::users::password_resetable::{PasswordResetable, PasswordReset};
use chrono::{Duration, Utc};
use diesel;
use diesel::prelude::*;
use support::project::TestProject;
use uuid::Uuid;

#[test]
fn find_by_password_reset_token() {
    let project = TestProject::new();
    let user = User::create("Jeff", "jeff@tari.com", "555-555-5555", "examplePassword")
        .commit(&project)
        .unwrap();
    let user = user.create_password_reset_token(&project).expect("Failed to create reset token");

    let found_user = User::find_by_password_reset_token(&user.password_reset_token.unwrap(), &project).expect("User was not found");
    assert_eq!(found_user.id, user.id);
    assert_eq!(found_user.password_reset_token.unwrap(), user.password_reset_token.unwrap());

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
    let user : User = user.create_password_reset_token(&project).expect("Failed to create reset token").into();
    let password = "newPassword";
    assert!(!user.check_password(&password));

    // Consumes password reset as token was not expired and valid
    let user = User::consume_password_reset_token(&user.password_reset_token.unwrap(), &password, &project).unwrap();
    assert!(user.check_password(&password));
    assert!(user.password_reset_token.is_none());
    assert!(user.password_reset_requested_at.is_none());

    // Does not consume password reset as token was expired although valid
    let user : User = diesel::update(users.filter(id.eq(user.id)))
        .set(
            PasswordReset {
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
fn has_valid_password_reset_token() {
    let project = TestProject::new();
    let mut user = User::create("Jeff", "jeff@tari.com", "555-555-5555", "examplePassword")
        .commit(&project)
        .unwrap();

    // Expired token
    user.password_reset_token = Some(Uuid::new_v4());
    user.password_reset_requested_at = Some(Utc::now().naive_utc() - Duration::days(1) - Duration::seconds(10));
    assert!(!user.has_valid_password_reset_token(), "Token should be expired");

    // Token not yet expired
    user.password_reset_token = Some(Uuid::new_v4());
    user.password_reset_requested_at = Some(Utc::now().naive_utc() - Duration::days(1) + Duration::seconds(10));
    assert!(user.has_valid_password_reset_token(), "Token should not be expired");

    // Token not yet expired
    user.password_reset_token = None;
    user.password_reset_requested_at = None;
    assert!(!user.has_valid_password_reset_token(), "Token does not exist so should be invalid");
}
