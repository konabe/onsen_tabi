use crate::domain::onsen::onsen_quality::OnsenQuality;
use crate::domain::onsen::{chemical::Chemical, onsen_entity::SpringLiquid};
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

impl DieselChemical {
    pub fn create(&self, liquid: Option<SpringLiquid>) -> OnsenQuality {
        let mut chemicals: Vec<Chemical> = vec![];
        if self.na_ion {
            chemicals.push(Chemical::NaIon);
        }
        if self.ca_ion {
            chemicals.push(Chemical::CaIon)
        }
        if self.mg_ion {
            chemicals.push(Chemical::MgIon)
        }
        if self.cl_ion {
            chemicals.push(Chemical::ClIon)
        }
        if self.hco3_ion {
            chemicals.push(Chemical::HCO3Ion)
        }
        if self.so4_ion {
            chemicals.push(Chemical::SO4Ion)
        }
        if self.co2_ion {
            chemicals.push(Chemical::CO2)
        }
        if self.fe_ion {
            chemicals.push(Chemical::FeIon(2))
        }
        if self.h_ion {
            chemicals.push(Chemical::HIon)
        }
        if self.i_ion {
            chemicals.push(Chemical::IIon)
        }
        if self.s {
            chemicals.push(Chemical::S)
        }
        if self.rn {
            chemicals.push(Chemical::Rn)
        }
        OnsenQuality::new(&chemicals, liquid)
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
