ALTER TABLE area ADD COLUMN national_resort tinyint(1) NOT NULL DEFAULT 0 AFTER prefecture;
ALTER TABLE area ADD COLUMN village varchar(255) AFTER national_resort;
