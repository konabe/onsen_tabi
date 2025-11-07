use std::str::FromStr;

use crate::domain::onsen::onsen_quality::OnsenQuality;
use crate::domain::onsen::spring_form::SpringForm;
use crate::domain::onsen::spring_liquid::SpringLiquid;
use crate::domain::onsen::spring_osmotic_pressure::SpringOsmoticPressure;
use crate::domain::onsen::spring_temperature::SpringTemperature;

#[derive(Clone)]
/// 温泉法が定義する温泉。
/// ◯◯温泉とは別
pub struct OnsenEntity {
    pub id: u32,
    /// 名前
    pub name: String,
    // TODO: "その他の泉質"を格納する場所を作る
    /// 泉質
    pub quality: Option<OnsenQuality>,
    /// 泉質名
    /// 泉質がNoneのときに名前を格納する
    pub spring_quality: String,
    /// 液性
    pub liquid: Option<SpringLiquid>,
    /// 浸透圧
    pub osmotic_pressure: Option<SpringOsmoticPressure>,
    /// 温度
    pub temperature: Option<SpringTemperature>,
    /// 営業形態
    pub form: SpringForm,
    /// 日帰り入浴可能か
    pub is_day_use: bool,
    /// URL
    pub url: String,
    /// 画像URL
    pub img_url: Option<String>,
    /// 説明
    pub description: String,
    /// エリアID
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
            name: String::new(),
            quality: None,
            spring_quality: String::new(),
            liquid: None,
            osmotic_pressure: None,
            temperature: None,
            form: SpringForm::default(),
            is_day_use: false,
            url: String::new(),
            img_url: None,
            description: String::new(),
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
        self.liquid = liquid.and_then(|v| SpringLiquid::from_str(v).ok());
        self
    }

    pub fn osmotic_pressure(&mut self, osmotic_pressure: Option<&str>) -> &mut Self {
        self.osmotic_pressure =
            osmotic_pressure.and_then(|v| SpringOsmoticPressure::from_str(v).ok());
        self
    }

    pub fn temperature(&mut self, temperature: Option<&str>) -> &mut Self {
        self.temperature = temperature.and_then(|v| SpringTemperature::from_str(v).ok());
        self
    }

    pub fn form(&mut self, form: &str) -> &mut Self {
        self.form = SpringForm::from_str(form).unwrap_or_default();
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

impl Default for OnsenEntityBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {

    use crate::domain::onsen::chemical::Chemical::*;
    use crate::domain::onsen::onsen_entity::{OnsenEntity, OnsenEntityBuilder};
    use crate::domain::onsen::onsen_quality::OnsenQuality;
    use crate::domain::onsen::spring_form::SpringForm;

    fn common_onsen_quality() -> OnsenQuality {
        OnsenQuality::new(&[NaIon, CaIon, SO4Ion], None)
    }

    #[test]
    fn build_test() {
        let onsen = OnsenEntityBuilder::new()
            .id(1)
            .name("元禄の湯")
            .quality(Some(common_onsen_quality()))
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
    fn build_test_return_none_when_name_is_empty() {
        let onsen = OnsenEntityBuilder::new()
            .id(1)
            .name("") // nameが空文字のとき
            .quality(Some(common_onsen_quality()))
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
        onsen.expect(""); // Noneになる。
    }

    #[test]
    fn build_test_return_uchiyu_when_form_is_entered_with_unknown_text() {
        let onsen = OnsenEntityBuilder::new()
            .id(1)
            .name("元禄の湯")
            .quality(Some(common_onsen_quality()))
            .spring_quality("ナトリウム・カルシウム 塩化物硫酸塩温泉")
            .liquid(Some("neutral"))
            .osmotic_pressure(Some("hypotonic"))
            .temperature(Some("hot"))
            .form("unknown") // 定義されてない文字列をformに入れる
            .is_day_use(true)
            .url("https://www.sekizenkan.co.jp/spa/#ank-spa1")
            .img_url(Some("https://placehold.jp/150x150.png"))
            .description("")
            .area_id(None)
            .build();
        let inside: OnsenEntity = onsen.expect("");
        assert!(inside.form == SpringForm::Uchiyu); // Uchiyuになる
    }
}
