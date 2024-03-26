use crate::domain::onsen::chemical::{ClType, FeType};
use crate::domain::onsen::onsen_quality::OnsenQuality;
use crate::domain::onsen::{chemical::Chemical, chemical::RnType, onsen_entity::SpringLiquid};
use diesel::{Identifiable, Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable, Identifiable, Insertable, Debug, Clone)]
#[diesel(table_name=crate::schema::chemicals)]
pub struct DieselChemical {
    pub id: u32,
    pub na_ion: u32,
    pub ca_ion: u32,
    pub mg_ion: u32,
    pub cl_ion: u32,
    pub hco3_ion: u32,
    pub so4_ion: u32,
    pub co2_ion: u32,
    pub fe_ion: u32,
    pub h_ion: u32,
    pub i_ion: u32,
    pub al_ion: u32,
    pub cu_ion: u32,
    pub s: u32,
    pub rn: u32,
    pub strong_na_cl: bool,
    pub fe_type: String,
    pub weak_rn: bool,
}

impl DieselChemical {
    pub fn create(&self, liquid: Option<SpringLiquid>) -> OnsenQuality {
        let cl_type: ClType = if self.strong_na_cl {
            ClType::Strong
        } else {
            ClType::Normal
        };
        let fe_type: FeType = if self.fe_type == "Two".to_string() {
            FeType::Two
        } else if self.fe_type == "Three".to_string() {
            FeType::Three
        } else {
            FeType::Normal
        };
        let rn_type: RnType = if self.weak_rn {
            RnType::Weak
        } else {
            RnType::Normal
        };
        let mut chemicals: Vec<(Chemical, u32)> = vec![
            (Chemical::NaIon, self.na_ion),
            (Chemical::CaIon, self.ca_ion),
            (Chemical::MgIon, self.mg_ion),
            (Chemical::ClIon(cl_type), self.cl_ion),
            (Chemical::HCO3Ion, self.hco3_ion),
            (Chemical::SO4Ion, self.so4_ion),
            (Chemical::CO2, self.co2_ion),
            (Chemical::FeIon(fe_type), self.fe_ion),
            (Chemical::AlIon, self.al_ion),
            (Chemical::CuIon, self.cu_ion),
            (Chemical::HIon, self.h_ion),
            (Chemical::IIon, self.i_ion),
            (Chemical::S, self.s),
            (Chemical::Rn(rn_type), self.rn),
        ]
        .into_iter()
        .filter(|(_, value)| *value > 0)
        .collect();
        chemicals.sort_by(|(_, a), (_, b)| a.cmp(b));
        let chemicals_values: Vec<Chemical> = chemicals
            .into_iter()
            .map(|(chemical, _)| chemical)
            .collect();
        OnsenQuality::new(&chemicals_values, liquid)
    }
}

impl From<OnsenQuality> for DieselChemical {
    fn from(value: OnsenQuality) -> Self {
        let mut self_ = Self {
            id: 0,
            na_ion: 0,
            ca_ion: 0,
            mg_ion: 0,
            cl_ion: 0,
            hco3_ion: 0,
            so4_ion: 0,
            co2_ion: 0,
            fe_ion: 0,
            al_ion: 0,
            cu_ion: 0,
            h_ion: 0,
            i_ion: 0,
            s: 0,
            rn: 0,
            strong_na_cl: value.is_strong_na_cl(),
            fe_type: value.fe_type(),
            weak_rn: value.is_weak_rn(),
        };
        for (i, v) in value.cations.iter().enumerate() {
            let index = i as u32;
            match v {
                Chemical::NaIon => self_.na_ion = index + 1,
                Chemical::CaIon => self_.ca_ion = index + 1,
                Chemical::MgIon => self_.mg_ion = index + 1,
                _ => (),
            }
        }
        for (i, v) in value.anions.iter().enumerate() {
            let index = i as u32;
            match v {
                Chemical::ClIon(_) => self_.cl_ion = index + 4,
                Chemical::HCO3Ion => self_.hco3_ion = index + 4,
                Chemical::SO4Ion => self_.so4_ion = index + 4,
                _ => (),
            }
        }
        for (i, v) in value.inclusions.iter().enumerate() {
            let index = i as u32;
            match v {
                Chemical::CO2 => self_.co2_ion = index + 7,
                Chemical::FeIon(_) => self_.fe_ion = index + 7,
                Chemical::AlIon => self_.al_ion = index + 7,
                Chemical::CuIon => self_.cu_ion = index + 7,
                Chemical::HIon => self_.h_ion = index + 7,
                Chemical::IIon => self_.i_ion = index + 7,
                Chemical::S => self_.s = index + 7,
                Chemical::Rn(_) => self_.rn = index + 7,
                _ => (),
            }
        }
        self_
    }
}
