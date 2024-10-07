use std::str::FromStr;

use crate::domain::onsen::onsen_quality::OnsenQuality;
use crate::domain::onsen::spring_form::SpringForm;
use crate::domain::onsen::spring_liquid::SpringLiquid;
use crate::domain::onsen::spring_osmotic_pressure::SpringOsmoticPressure;
use crate::domain::onsen::spring_temperature::SpringTemperature;

/// 温泉法が定義する温泉。
/// ◯◯温泉とは別
#[derive(Clone)]
pub struct OnsenEntity {
    pub id: u32,
    pub name: String,
    // TODO: "その他の泉質"を格納する場所を作る
    pub quality: Option<OnsenQuality>,
    pub spring_quality: String,
    pub liquid: Option<SpringLiquid>,
    pub osmotic_pressure: Option<SpringOsmoticPressure>,
    pub temperature: Option<SpringTemperature>,
    pub form: SpringForm,
    pub is_day_use: bool,
    pub url: String,
    pub img_url: Option<String>,
    pub description: String,
    pub area_id: Option<u32>,
}

// pubでプロパティを作って
pub struct OnsenEntityBuilder {
    id: u32,
    name: String,
    quality: Option<OnsenQuality>,
    spring_quality: String,
    liquid: Option<SpringLiquid>,
    osmotic_pressure: Option<SpringOsmoticPressure>,
    temperature: Option<SpringTemperature>,
    form: SpringForm,
    is_day_use: bool,
    url: String,
    img_url: Option<String>,
    description: String,
    area_id: Option<u32>,
}

impl OnsenEntityBuilder {
    pub fn new() -> Self {
        Self {
            id: 0,
            name: "".to_string(),
            quality: None,
            spring_quality: "".to_string(),
            liquid: None,
            osmotic_pressure: None,
            temperature: None,
            form: SpringForm::Uchiyu,
            is_day_use: false,
            url: "".to_string(),
            img_url: None,
            description: "".to_string(),
            area_id: None,
        }
    }

    pub fn id(&mut self, id: u32) -> &mut Self {
        self.id = id;
        self
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = name.to_string();
        self
    }

    pub fn quality(&mut self, quality: Option<OnsenQuality>) -> &mut Self {
        self.quality = quality;
        self
    }

    pub fn spring_quality(&mut self, spring_quality: &str) -> &mut Self {
        self.spring_quality = spring_quality.to_string();
        self
    }

    pub fn liquid(&mut self, liquid: Option<&str>) -> &mut Self {
        let liquid = liquid.and_then(|v| SpringLiquid::from_str(v).ok());
        self.liquid = liquid;
        self
    }

    pub fn osmotic_pressure(&mut self, osmotic_pressure: Option<&str>) -> &mut Self {
        let osmotic_pressure =
            osmotic_pressure.and_then(|v| SpringOsmoticPressure::from_str(v).ok());
        self.osmotic_pressure = osmotic_pressure;
        self
    }

    pub fn temperature(&mut self, temperature: Option<&str>) -> &mut Self {
        let temperature = temperature.and_then(|v| SpringTemperature::from_str(v).ok());
        self.temperature = temperature;
        self
    }

    pub fn form(&mut self, form: &str) -> &mut Self {
        let form = SpringForm::from_str(form).ok().unwrap_or_default();
        self.form = form;
        self
    }

    pub fn is_day_use(&mut self, is_day_use: bool) -> &mut Self {
        self.is_day_use = is_day_use;
        self
    }

    pub fn url(&mut self, url: &str) -> &mut Self {
        self.url = url.to_string();
        self
    }

    pub fn img_url(&mut self, img_url: Option<&str>) -> &mut Self {
        self.img_url = img_url.map(|v| v.to_string());
        self
    }

    pub fn description(&mut self, description: &str) -> &mut Self {
        self.description = description.to_string();
        self
    }

    pub fn area_id(&mut self, area_id: Option<u32>) -> &mut Self {
        self.area_id = area_id;
        self
    }

    pub fn build(&self) -> Option<OnsenEntity> {
        if self.name.is_empty() {
            return None;
        }
        Some(OnsenEntity {
            id: self.id,
            name: self.name.clone(),
            quality: self.quality.clone(),
            spring_quality: self.spring_quality.clone(),
            liquid: self.liquid.clone(),
            osmotic_pressure: self.osmotic_pressure.clone(),
            temperature: self.temperature.clone(),
            form: self.form.clone(),
            is_day_use: self.is_day_use,
            url: self.url.clone(),
            img_url: self.img_url.clone(),
            description: self.description.clone(),
            area_id: self.area_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;

    use crate::domain::onsen::chemical::Chemical::*;
    use crate::domain::onsen::onsen_entity::{OnsenEntity, OnsenEntityBuilder};
    use crate::domain::onsen::onsen_quality::OnsenQuality;

    const COMMON_ONSEN_QUALITY: Lazy<OnsenQuality> =
        Lazy::new(|| OnsenQuality::new(&vec![NaIon, CaIon, SO4Ion], None));

    #[test]
    fn new_test() {
        let onsen = OnsenEntityBuilder::new()
            .id(1)
            .name("元禄の湯")
            .quality(Some(COMMON_ONSEN_QUALITY.clone()))
            .spring_quality("ナトリウム・カルシウム 塩化物硫酸塩温泉")
            .liquid(Some("neutral"))
            .osmotic_pressure(Some("hypotonic"))
            .temperature(Some("hot"))
            .form("uchiyu")
            .is_day_use(true)
            .url("https://www.sekizenkan.co.jp/spa/#ank-spa1")
            .img_url(Some("https://placehold.jp/150x150.png"))
            .description("")
            .area_id(None)
            .build();
        let inside: OnsenEntity = onsen.expect("");
        assert!(inside.name == "元禄の湯");
    }

    #[test]
    #[should_panic]
    fn new_test_return_none_when_name_is_empty() {
        let onsen = OnsenEntityBuilder::new()
            .id(1)
            .name("")
            .quality(Some(COMMON_ONSEN_QUALITY.clone()))
            .spring_quality("ナトリウム・カルシウム 塩化物硫酸塩温泉")
            .liquid(Some("neutral"))
            .osmotic_pressure(Some("hypotonic"))
            .temperature(Some("hot"))
            .form("uchiyu")
            .is_day_use(true)
            .url("https://www.sekizenkan.co.jp/spa/#ank-spa1")
            .img_url(Some("https://placehold.jp/150x150.png"))
            .description("")
            .area_id(None)
            .build();
        onsen.expect("");
    }
}
