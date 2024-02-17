use crate::domain::onsen::chemical::Chemical;
use crate::domain::onsen::onsen_quality::OnsenQuality;
use diesel::{Identifiable, Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable, Identifiable, Insertable, Debug, Clone)]
#[diesel(table_name=crate::schema::chemicals)]
pub struct DieselChemical {
    pub id: u32,
    pub na_ion: bool,
    pub ca_ion: bool,
    pub mg_ion: bool,
    pub cl_ion: bool,
    pub hco3_ion: bool,
    pub so4_ion: bool,
    pub co2_ion: bool,
    pub fe_ion: bool,
    pub h_ion: bool,
    pub i_ion: bool,
    pub s: bool,
    pub rn: bool,
}

impl From<DieselChemical> for OnsenQuality {
    fn from(value: DieselChemical) -> Self {
        let mut chemicals: Vec<Chemical> = vec![];
        if value.na_ion {
            chemicals.push(Chemical::NaIon);
        }
        if value.ca_ion {
            chemicals.push(Chemical::CaIon)
        }
        if value.mg_ion {
            chemicals.push(Chemical::MgIon)
        }
        if value.cl_ion {
            chemicals.push(Chemical::ClIon)
        }
        if value.hco3_ion {
            chemicals.push(Chemical::HCO3Ion)
        }
        if value.so4_ion {
            chemicals.push(Chemical::SO4Ion)
        }
        if value.co2_ion {
            chemicals.push(Chemical::CO2)
        }
        if value.fe_ion {
            chemicals.push(Chemical::FeIon(2))
        }
        if value.h_ion {
            chemicals.push(Chemical::HIon)
        }
        if value.i_ion {
            chemicals.push(Chemical::IIon)
        }
        if value.s {
            chemicals.push(Chemical::S)
        }
        if value.rn {
            chemicals.push(Chemical::Rn)
        }
        Self::new(&chemicals, None)
    }
}

impl From<OnsenQuality> for DieselChemical {
    fn from(value: OnsenQuality) -> Self {
        Self {
            id: 0,
            na_ion: value.cations.contains(&Chemical::NaIon),
            ca_ion: value.cations.contains(&Chemical::CaIon),
            mg_ion: value.cations.contains(&Chemical::MgIon),
            cl_ion: value.anions.contains(&Chemical::ClIon),
            hco3_ion: value.anions.contains(&Chemical::HCO3Ion),
            so4_ion: value.anions.contains(&Chemical::SO4Ion),
            co2_ion: value.inclusions.contains(&Chemical::CO2),
            fe_ion: value.inclusions.contains(&Chemical::FeIon(2)),
            h_ion: value.inclusions.contains(&Chemical::HIon),
            i_ion: value.inclusions.contains(&Chemical::IIon),
            s: value.inclusions.contains(&Chemical::S),
            rn: value.inclusions.contains(&Chemical::Rn),
        }
    }
}
