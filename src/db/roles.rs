pub const GUEST: u8 = 0;
pub const USER: u8 = 1;
pub const ORG_MEMBER: u8 = 2;
pub const ORG_OWNER: u8 = 3;
pub const ADMIN: u8 = 4;
pub const NUM_ROLES: u8 = 5;

// Diesel doesn't support enums cleanly yet, so this is a bit of a hacky map
pub fn get_role_name(role: u8) -> String {
    let str = match role {
        USER => "bigneon_user",
        ORG_MEMBER => "bigneon_orgmember",
        ORG_OWNER => "bigneon_orgowner",
        ADMIN => "bigneon_admin",
        _ => "bigneon_guest",
    };
    str.into()
}

#[test]
fn test_get_role_name() {
    assert_eq!(get_role_name(USER), "bigneon_user");
    assert_eq!(get_role_name(GUEST), "bigneon_guest");
    assert_eq!(get_role_name(ORG_MEMBER), "bigneon_orgmember");
    assert_eq!(get_role_name(ORG_OWNER), "bigneon_orgowner");
    assert_eq!(get_role_name(ADMIN), "bigneon_admin");
}
