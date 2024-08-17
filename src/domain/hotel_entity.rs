use super::onsen::onsen_entity::OnsenEntity;

#[derive(Clone)]
pub struct HotelEntity {
    pub id: u32,
    pub name: String,
    pub has_washitsu: bool,
    pub solo_available: bool,
    pub url: String,
    pub description: String,
    pub onsens: Vec<OnsenEntity>,
}

#[derive(Clone, Default)]
pub struct HotelEntityBuilder {
    pub id: u32,
    pub name: String,
    pub has_washitsu: bool,
    pub solo_available: bool,
    pub url: String,
    pub description: String,
    pub onsens: Vec<OnsenEntity>,
}

impl HotelEntityBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: u32) -> Self {
        self.id = id;
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn has_washitsu(mut self, has_washitsu: bool) -> Self {
        self.has_washitsu = has_washitsu;
        self
    }

    pub fn solo_available(mut self, solo_available: bool) -> Self {
        self.solo_available = solo_available;
        self
    }

    pub fn url(mut self, url: &str) -> Self {
        self.url = url.to_string();
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    pub fn onsens(mut self, onsens: Vec<OnsenEntity>) -> Self {
        self.onsens = onsens;
        self
    }

    pub fn build(self) -> Option<HotelEntity> {
        if self.name.is_empty() {
            return None;
        }
        Some(HotelEntity {
            id: self.id,
            name: self.name,
            has_washitsu: self.has_washitsu,
            solo_available: self.solo_available,
            url: self.url,
            description: self.description,
            onsens: self.onsens,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::{
        hotel_entity::{HotelEntity, HotelEntityBuilder},
        onsen::onsen_entity::OnsenEntity,
    };
    use once_cell::sync::Lazy;

    const COMMON_ONSEN: Lazy<OnsenEntity> = Lazy::new(|| {
        OnsenEntity::new(
            1,
            "積善館 元禄の湯",
            None,
            "単純温泉",
            Some("neutral"),
            Some("isotonic"),
            Some("hot"),
            "sotoyu",
            true,
            "https://www.sekizenkan.co.jp/spa/#ank-spa1",
            Some("https://placehold.jp/150x150.png"),
            "",
            None,
        )
        .expect("")
    });

    #[test]
    fn new_and_clone_test() {
        let hotel = HotelEntityBuilder::new()
            .id(1)
            .name("積善館")
            .has_washitsu(true)
            .solo_available(false)
            .url("https://www.sekizenkan.co.jp/")
            .description("")
            .onsens(vec![COMMON_ONSEN.clone()])
            .build();
        let hotel: HotelEntity = hotel.expect("");
        assert!(hotel.name == "積善館");
        assert!(hotel.has_washitsu == true);
        let cloned_hotel = hotel.clone();
        assert!(cloned_hotel.name == "積善館");
        assert!(cloned_hotel.has_washitsu == true);
    }

    #[test]
    #[should_panic]
    fn new_test_return_none_when_name_is_empty() {
        let hotel = HotelEntityBuilder::new()
            .id(1)
            .name("")
            .has_washitsu(true)
            .solo_available(true)
            .url("https://www.sekizenkan.co.jp/")
            .description("")
            .onsens(vec![])
            .build();
        hotel.unwrap();
    }
}
