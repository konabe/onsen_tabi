ALTER TABLE user ADD COLUMN salt varchar(255) NOT NULL DEFAULT "" AFTER hashed_password;
