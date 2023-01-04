DROP TABLE IF EXISTS groups CASCADE;
CREATE TABLE groups (
  group_id SERIAL,
  status VARCHAR(100) NOT NULL DEFAULT 'open',
  PRIMARY KEY (group_id)
);
