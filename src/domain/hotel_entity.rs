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

impl HotelEntity {
    pub fn new(
        id: u32,
        name: &str,
        has_washitsu: bool,
        solo_available: bool,
        url: &str,
        description: &str,
        onsens: &[OnsenEntity],
    ) -> Option<Self> {
        if name.is_empty() {
            return None;
        }
        Some(Self {
            id,
            name: name.to_string(),
            has_washitsu,
            solo_available,
            url: url.to_string(),
            description: description.to_string(),
            onsens: onsens.to_vec(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::{hotel_entity::HotelEntity, onsen::onsen_entity::OnsenEntity};
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
        let hotel = HotelEntity::new(
            1,
            "積善館",
            true,
            false,
            "https://www.sekizenkan.co.jp/",
            "",
            &vec![COMMON_ONSEN.clone()],
        );
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
        let hotel = HotelEntity::new(
            1,
            "",
            true,
            true,
            "https://www.sekizenkan.co.jp/",
            "",
            &vec![],
        );
        hotel.unwrap();
    }
}
