-- Define the organization_invites table
CREATE TABLE organization_invites (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
  organization_id uuid NOT NULL REFERENCES organizations (id),
  invitee_id uuid NOT NULL REFERENCES users (id),
  created_on TIMESTAMP NOT NULL,
  security_token uuid,
  user_id uuid REFERENCES users (id),
  status_changed_on TIMESTAMP,
  accepted SMALLINT,
  user_email TEXT
);

-- Indices
CREATE INDEX index_organization_invites_organization_id ON organization_invites (organization_id);
CREATE INDEX index_organization_invites_user_id ON organization_invites (organization_id);
CREATE INDEX index_organization_invitee_user_id ON organization_invites (user_id);
