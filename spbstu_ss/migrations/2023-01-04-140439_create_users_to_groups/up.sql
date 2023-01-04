DROP TABLE IF EXISTS user_to_group CASCADE;

CREATE TABLE user_to_group (
  group_id INTEGER NOT NULL,
  user_id INTEGER NOT NULL,
  role VARCHAR(100) NOT NULL DEFAULT 'member',
  PRIMARY KEY (user_id, group_id)
);

ALTER TABLE user_to_group ADD FOREIGN KEY (group_id) REFERENCES groups (group_id);
ALTER TABLE user_to_group ADD FOREIGN KEY (user_id) REFERENCES users (user_id);
