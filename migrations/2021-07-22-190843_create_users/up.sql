-- Your SQL goes here
CREATE TABLE users(
  id   		 SERIAL  PRIMARY KEY,
  created_on TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_on TIMESTAMP NOT NULL DEFAULT NOW(),
  email 	 VARCHAR(32) NOT NULL UNIQUE,
  first_name VARCHAR(32) NOT NULL,
  last_name  VARCHAR(32),
  password 	 VARCHAR(64) NOT NULL
)
