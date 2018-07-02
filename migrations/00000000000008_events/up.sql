-- Define the events table
CREATE TABLE events (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
  organization_id uuid NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
  venue_id uuid NOT NULL REFERENCES venues (id) ON DELETE CASCADE
);

-- In general users have READ_ONLY access to the events table
REVOKE ALL ON events FROM PUBLIC;

GRANT SELECT, INSERT, UPDATE ON events TO bigneon_admin;
GRANT SELECT, UPDATE, INSERT ON events TO bigneon_orgowner, bigneon_orgmember;
GRANT SELECT ON events TO bigneon_user;

-- Indices
CREATE INDEX index_events_organization_id ON events (organization_id);
CREATE INDEX index_events_venue_id ON events (venue_id);
