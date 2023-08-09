CREATE TABLE IF NOT EXISTS area (
  id int unsigned NOT NULL AUTO_INCREMENT,
  name varchar(255) NOT NULL,
  PRIMARY KEY (id)
);

ALTER TABLE hotel ADD COLUMN area_id int unsigned AFTER has_washitsu;
ALTER TABLE hotel ADD FOREIGN KEY fk_hotel_area_id(area_id) REFERENCES area(id);

ALTER TABLE onsen ADD COLUMN area_id int unsigned AFTER hotel_id;
ALTER TABLE onsen ADD FOREIGN KEY fk_onsen_area_id(area_id) REFERENCES area(id);
