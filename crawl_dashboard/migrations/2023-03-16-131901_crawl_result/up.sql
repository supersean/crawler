-- Your SQL goes here

CREATE TABLE users(
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

CREATE TABLE entries(
  id SERIAL PRIMARY KEY,
  terms VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  user_id INTEGER NOT NULL REFERENCES users
);

CREATE TABLE crawl_results (
  id SERIAL PRIMARY KEY,
  url VARCHAR NOT NULL,
  raw_html VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  entry_id INTEGER NOT NULL REFERENCES entries
);

