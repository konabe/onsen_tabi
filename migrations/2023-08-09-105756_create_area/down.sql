ALTER TABLE hotel DROP CONSTRAINT fk_hotel_area_id;
ALTER TABLE hotel DROP COLUMN area_id;
ALTER TABLE onsen DROP CONSTRAINT fk_onsen_area_id;
ALTER TABLE onsen ADD COLUMN area_id;
DROP TABLE area;