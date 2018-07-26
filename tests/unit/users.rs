use bigneon_db::models::Roles;
use bigneon_db::models::User;
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
    let user = project.create_user().finish();

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
fn for_display() {
    let project = TestProject::new();
    let user = project.create_user().finish();
    let user_id = user.id.clone();
    let display_user = user.for_display();

    assert_eq!(display_user.id, user_id);
}

#[test]
fn add_role() {
    let project = TestProject::new();
    let user = project.create_user().finish();

    user.add_role(Roles::Admin, &project).unwrap();

    let user2 = User::find(&user.id, &project).unwrap();
    assert_eq!(user2.role, vec!["Guest", "Admin"]);
}
