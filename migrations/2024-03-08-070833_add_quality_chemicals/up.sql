ALTER TABLE chemicals ADD COLUMN al_ion int unsigned NOT NULL DEFAULT 0 AFTER fe_ion;
ALTER TABLE chemicals ADD COLUMN cu_ion int unsigned NOT NULL DEFAULT 0 AFTER al_ion;
ALTER TABLE chemicals ADD COLUMN strong_na_cl tinyint(1) NOT NULL DEFAULT 0 AFTER rn;
ALTER TABLE chemicals ADD COLUMN weak_rn tinyint(1) NOT NULL DEFAULT 0 AFTER strong_na_cl;
