CREATE EXTENSION pgcrypto;

CREATE TABLE collections(
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name text NOT NULL,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL
);
