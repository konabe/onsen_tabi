CREATE TABLE IF NOT EXISTS onsen (
  id int unsigned NOT NULL AUTO_INCREMENT,
  name varchar(255) NOT NULL,
  spring_quality varchar(255) NOT NULL,
  liquid varchar(255) NOT NULL,
  osmotic_pressure varchar(255) NOT NULL,
  category varchar(255) NOT NULL,
  hotel_id int unsigned DEFAULT NULL,
  PRIMARY KEY (id),
  KEY hotel_id (hotel_id),
  CONSTRAINT onsen_ibfk_1 FOREIGN KEY (hotel_id) REFERENCES hotel (id) ON DELETE RESTRICT
);