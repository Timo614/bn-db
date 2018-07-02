-- Define the organization_users table
CREATE TABLE organization_users (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
  organization_id uuid NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
  user_id uuid NOT NULL REFERENCES users (id) ON DELETE CASCADE
);

-- In general users have READ_ONLY access to the organization_users table
REVOKE ALL ON organization_users FROM PUBLIC;

GRANT SELECT, INSERT, UPDATE ON organization_users TO bigneon_admin;
GRANT SELECT, INSERT, UPDATE ON organization_users TO bigneon_orgowner;
GRANT SELECT ON organization_users TO bigneon_orgmember, bigneon_user;

-- Indices
CREATE INDEX index_organization_users_organization_id_user_id ON organization_users (organization_id,user_id);
