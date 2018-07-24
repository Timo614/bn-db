use bigneon_db::models::User;
use support::project::TestProject;

use rand::prelude::*;

pub struct UserBuilder<'a> {
    name: String,
    email: String,
    phone: String,
    password: String,
    test_project: &'a TestProject,
}

impl<'a> UserBuilder<'a> {
    pub fn new(test_project: &TestProject) -> UserBuilder {
        let x: u8 = random();

        UserBuilder {
            name: "Jeff".into(),
            email: format!("jeff{}@tari.com", x).into(),
            phone: "555-555-5555".into(),
            password: "examplePassword".into(),
            test_project,
        }
    }

    pub fn finish(&self) -> User {
        User::create(&self.name, &self.email, &self.phone, &self.password)
            .commit(self.test_project)
            .unwrap()
    }
}
