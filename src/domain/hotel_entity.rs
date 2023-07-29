pub struct HotelEntity {
    pub id: u32,
    pub name: String,
    pub has_washitsu: bool,
}

impl HotelEntity {
    pub fn new(id: u32, name: &str, has_washitsu: bool) -> Option<Self> {
        if name.is_empty() {
            return None;
        }
        return Some(Self {
            id,
            name: name.to_string(),
            has_washitsu,
        });
    }
}

#[test]
fn new_test() {
    let hotel = HotelEntity::new(1, "積善館", true);
    let inside: HotelEntity = hotel.expect("");
    assert!(inside.name == "積善館");
    assert!(inside.has_washitsu == true);
}

#[test]
#[should_panic]
fn new_test_none() {
    let hotel = HotelEntity::new(1, "", true);
    hotel.unwrap();
}
