-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email TEXT NOT NULL,
  hashed_pw TEXT NOT NULL,
  active BOOLEAN NOT NULL DEFAULT 't'
)