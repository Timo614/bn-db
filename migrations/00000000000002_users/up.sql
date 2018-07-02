-- Define the users table
CREATE TABLE users (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
  name TEXT NOT NULL,
  email TEXT NOT NULL UNIQUE,
  phone TEXT NOT NULL,
  hashed_pw TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT now(),
  last_used TIMESTAMP DEFAULT NULL,
  active BOOLEAN NOT NULL DEFAULT 't',
  role integer NOT NULL DEFAULT 1 -- Registered user
);

-- In general users have READ_ONLY access to the users table
REVOKE ALL ON users FROM PUBLIC;

GRANT SELECT, INSERT, UPDATE ON users TO bigneon_admin;
GRANT SELECT, UPDATE, INSERT ON users TO bigneon_orgowner;
GRANT SELECT, UPDATE ON users TO bigneon_orgmember, bigneon_user;

-- Indices
CREATE INDEX index_users_uuid ON users (id);
CREATE INDEX index_users_email ON users (email);