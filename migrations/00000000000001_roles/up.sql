CREATE OR REPLACE FUNCTION create_role_if_not_exists(rolename NAME) RETURNS TEXT AS
$$
BEGIN
    IF NOT EXISTS (SELECT * FROM pg_roles WHERE rolname = rolename) THEN
        EXECUTE format('CREATE ROLE %I NOINHERIT LOGIN', rolename);
        RETURN 'CREATE ROLE';
    ELSE
        RETURN format('ROLE ''%I'' ALREADY EXISTS', rolename);
    END IF;
END;
$$
LANGUAGE plpgsql;

-- The BN database "admin". Can modify most aspects of the database
SELECT create_role_if_not_exists('bigneon_admin');

-- An Organisation owner. Can manage other Organisation admins and can modify org details
SELECT create_role_if_not_exists('bigneon_orgowner');

-- An Organisation admin. Can create and manage Events
SELECT create_role_if_not_exists('bigneon_orgmember');

-- An authenticated user. Can buy tickets, manage own data and view own history
SELECT create_role_if_not_exists('bigneon_user');

-- An unauthenticated user. Read-only access to some tables
SELECT create_role_if_not_exists('bigneon_guest');

CREATE OR REPLACE FUNCTION is_admin_role(v int) RETURNS boolean AS 'select v=4;' LANGUAGE SQL IMMUTABLE LEAKPROOF;
CREATE OR REPLACE FUNCTION is_orgowner_role(v int) RETURNS boolean AS 'select v=3;' LANGUAGE SQL IMMUTABLE LEAKPROOF;
CREATE OR REPLACE FUNCTION is_orgmember_role(v int) RETURNS boolean AS 'select v=2;' LANGUAGE SQL IMMUTABLE LEAKPROOF;
CREATE OR REPLACE FUNCTION is_user_role(v int) RETURNS boolean AS 'select v=1;' LANGUAGE SQL IMMUTABLE LEAKPROOF;
CREATE OR REPLACE FUNCTION is_guest_role(v int) RETURNS boolean AS 'select v=0;' LANGUAGE SQL IMMUTABLE LEAKPROOF;

