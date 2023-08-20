CREATE TABLE IF NOT EXISTS user (
  id int unsigned NOT NULL AUTO_INCREMENT,
  name varchar(255) NOT NULL,
  has_washitsu tinyint(1) NOT NULL DEFAULT 0,
  PRIMARY KEY (id)
);