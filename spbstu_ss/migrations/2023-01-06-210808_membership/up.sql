DROP TABLE IF EXISTS memberships CASCADE;

CREATE TABLE memberships (
  group_id INTEGER NOT NULL,
  user_id INTEGER NOT NULL,
  role VARCHAR(100) NOT NULL DEFAULT 'member',
  PRIMARY KEY (user_id, group_id)
);

ALTER TABLE memberships ADD FOREIGN KEY (group_id) REFERENCES groups (group_id) ON DELETE CASCADE;
ALTER TABLE memberships ADD FOREIGN KEY (user_id) REFERENCES users (user_id) ON DELETE CASCADE;
