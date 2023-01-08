DROP TABLE IF EXISTS users CASCADE;
DROP VIEW IF EXISTS users_wo_passwords;

CREATE TABLE users (
  user_id SERIAL,
  name VARCHAR(100) NOT NULL UNIQUE,
  password VARCHAR(100) NOT NULL,
  PRIMARY KEY (user_id)
);

CREATE VIEW users_wo_passwords AS
    SELECT user_id, name FROM users;
