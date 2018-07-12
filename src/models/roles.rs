use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;

pub enum Roles {
    Admin,
    Guest,
    OrgMember,
    OrgOwner,
    User,
}

impl Display for Roles {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Roles::Guest => write!(f, "Guest"),
            Roles::User => write!(f, "User"),
            Roles::OrgMember => write!(f, "OrgMember"),
            Roles::OrgOwner => write!(f, "OrgOwner"),
            Roles::Admin => write!(f, "Admin"),
        }
    }
}

#[test]
fn display() {
    assert_eq!(Roles::Admin.to_string(), "Admin");
    assert_eq!(Roles::Guest.to_string(), "Guest");
    assert_eq!(Roles::OrgMember.to_string(), "OrgMember");
    assert_eq!(Roles::OrgOwner.to_string(), "OrgOwner");
    assert_eq!(Roles::User.to_string(), "User");
}
