use super::onsen_entity::OnsenEntity;

#[derive(Clone)]
pub struct HotelEntity {
    pub id: u32,
    pub name: String,
    pub has_washitsu: bool,
    pub url: String,
    pub onsens: Vec<OnsenEntity>,
}

impl HotelEntity {
    pub fn new(
        id: u32,
        name: &str,
        has_washitsu: bool,
        url: &str,
        onsens: &[OnsenEntity],
    ) -> Option<Self> {
        if name.is_empty() {
            return None;
        }
        Some(Self {
            id,
            name: name.to_string(),
            has_washitsu,
            url: url.to_string(),
            onsens: onsens.to_vec(),
        })
    }
}

#[test]
fn new_test() {
    let hotel = HotelEntity::new(
        1,
        "積善館",
        true,
        "https://www.sekizenkan.co.jp/",
        &vec![OnsenEntity::new(
            1,
            "テスト温泉",
            "単純温泉",
            Some("neutral"),
            Some("isotonic"),
            "sotoyu",
            "https://www.sekizenkan.co.jp/spa/#ank-spa1",
            "",
        )
        .expect("")],
    );
    let inside: HotelEntity = hotel.expect("");
    assert!(inside.name == "積善館");
    assert!(inside.has_washitsu == true);
}

#[test]
#[should_panic]
fn new_test_none() {
    let hotel = HotelEntity::new(1, "", true, "https://www.sekizenkan.co.jp/", &vec![]);
    hotel.unwrap();
}
