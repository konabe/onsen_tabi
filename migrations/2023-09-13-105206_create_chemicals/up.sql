CREATE TABLE IF NOT EXISTS chemicals (
  id int unsigned NOT NULL AUTO_INCREMENT,
  na_ion tinyint(1) NOT NULL DEFAULT 0,
  ca_ion tinyint(1) NOT NULL DEFAULT 0,
  mg_ion tinyint(1) NOT NULL DEFAULT 0,
  cl_ion tinyint(1) NOT NULL DEFAULT 0,
  hco3_ion tinyint(1) NOT NULL DEFAULT 0,
  so4_ion tinyint(1) NOT NULL DEFAULT 0,
  co2_ion tinyint(1) NOT NULL DEFAULT 0,
  fe_ion tinyint(1) NOT NULL DEFAULT 0,
  h_ion tinyint(1) NOT NULL DEFAULT 0,
  i_ion tinyint(1) NOT NULL DEFAULT 0,
  s tinyint(1) NOT NULL DEFAULT 0,
  rn tinyint(1) NOT NULL DEFAULT 0,
  PRIMARY KEY (id)
);

ALTER TABLE onsen ADD COLUMN chemical_id int unsigned AFTER description;
ALTER TABLE onsen ADD FOREIGN KEY fk_onsen_chemical_id(chemical_id) REFERENCES chemicals(id);
