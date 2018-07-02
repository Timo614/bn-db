-- Define the orders table
CREATE TABLE orders (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
  user_id uuid NOT NULL REFERENCES users (id) ON DELETE CASCADE,
  event_id uuid NOT NULL REFERENCES events (id) ON DELETE CASCADE
);

-- In general users have READ_ONLY access to the organization_venues table
REVOKE ALL ON orders FROM PUBLIC;

GRANT SELECT, INSERT, UPDATE ON orders TO bigneon_admin;
GRANT SELECT, UPDATE, INSERT ON orders TO bigneon_orgowner, bigneon_orgmember;
GRANT SELECT ON orders TO bigneon_user;

-- Indices
CREATE INDEX index_orders_user_id ON orders (user_id);
CREATE INDEX index_orders_event_id ON orders (event_id);
