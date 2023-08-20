-- Your SQL goes here
CREATE TABLE IF NOT EXISTS user (
  id int unsigned NOT NULL AUTO_INCREMENT,
  email varchar(255) NOT NULL,
  hashed_password varchar(255) NOT NULL,
  role varchar(255) NOT NULL,
  PRIMARY KEY (id)
);