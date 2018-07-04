-- Define the artists table
CREATE TABLE artists (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
  name TEXT NOT NULL
);


-- In general users have READ_ONLY access to the organization_venues table
REVOKE ALL ON artists FROM PUBLIC;

GRANT SELECT, INSERT, UPDATE ON artists TO bigneon_admin;
GRANT SELECT ON artists TO bigneon_orgowner, bigneon_orgmember, bigneon_user;
