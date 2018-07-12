-- Define the events table
CREATE TABLE events (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
  organization_id uuid NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
  venue_id uuid NOT NULL REFERENCES venues (id) ON DELETE CASCADE
);

-- Indices
CREATE INDEX index_events_organization_id ON events (organization_id);
CREATE INDEX index_events_venue_id ON events (venue_id);
