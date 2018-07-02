-- Define the venues table
CREATE TABLE venues (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
  name TEXT NOT NULL
);

-- In general venues have READ_ONLY access to the venues table
REVOKE ALL ON venues FROM PUBLIC;

GRANT SELECT, INSERT, UPDATE ON venues TO bigneon_admin;
GRANT SELECT, UPDATE, INSERT ON venues TO bigneon_orgowner, bigneon_orgmember;
GRANT SELECT ON venues TO bigneon_user;
