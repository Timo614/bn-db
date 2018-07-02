-- Define the event_artists table
CREATE TABLE event_artists (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
  event_id uuid NOT NULL REFERENCES events (id) ON DELETE CASCADE,
  artist_id uuid NOT NULL REFERENCES artists (id) ON DELETE CASCADE,
  rank INTEGER NOT NULL
);

-- In general users have READ_ONLY access to the event_artists table
REVOKE ALL ON event_artists FROM PUBLIC;

GRANT SELECT, INSERT, UPDATE ON event_artists TO bigneon_admin;
GRANT SELECT, UPDATE, INSERT ON event_artists TO bigneon_orgowner, bigneon_orgmember;
GRANT SELECT ON event_artists TO bigneon_user;

-- Indices
CREATE INDEX index_event_artists_event_id ON event_artists (event_id);
CREATE INDEX index_event_artists_artist_id ON event_artists (artist_id);
