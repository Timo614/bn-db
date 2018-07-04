-- Define the organization_venues table
CREATE TABLE organization_venues (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
  organization_id uuid NOT NULL REFERENCES organizations (id),
  venue_id uuid NOT NULL REFERENCES venues (id)
);

-- In general users have READ_ONLY access to the organization_venues table
REVOKE ALL ON organization_venues FROM PUBLIC;

GRANT SELECT, UPDATE, INSERT ON organization_venues TO bigneon_admin;
GRANT SELECT, UPDATE, INSERT ON organization_venues TO bigneon_orgowner, bigneon_orgmember;
GRANT SELECT ON organization_venues TO bigneon_user;

-- Indices
CREATE INDEX index_organization_venues_organization_id_venue_id ON organization_venues (organization_id,venue_id);
