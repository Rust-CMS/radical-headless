-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE pages (
  uuid UUID NOT NULL DEFAULT uuid_generate_v1(),
  page_title VARCHAR,
  slug VARCHAR,

  CONSTRAINT uuid_tbl PRIMARY KEY ( uuid )
)