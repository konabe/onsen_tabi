ALTER TABLE onsen ADD COLUMN url varchar(255) NOT NULL DEFAULT "" AFTER category;
ALTER TABLE hotel ADD COLUMN url varchar(255) NOT NULL DEFAULT "" AFTER has_washitsu;
ALTER TABLE area ADD COLUMN url varchar(255) NOT NULL DEFAULT "" AFTER prefecture;
