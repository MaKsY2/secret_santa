DROP TABLE IF EXISTS santas CASCADE;

CREATE TABLE santas (
  group_id INTEGER NOT NULL,
  santa_user_id INTEGER NOT NULL,
  reciever_user_id INTEGER NOT NULL,
  PRIMARY KEY (group_id, santa_user_id)
);

ALTER TABLE santas ADD FOREIGN KEY (group_id) REFERENCES groups (group_id);
ALTER TABLE santas ADD FOREIGN KEY (santa_user_id) REFERENCES users (user_id);
ALTER TABLE santas ADD FOREIGN KEY (reciever_user_id) REFERENCES users (user_id);
