pub struct AreaEntity {
    pub id: u32,
    pub name: String,
}

impl AreaEntity {
    pub fn new(id: u32, name: &str) -> Option<Self> {
        if name.is_empty() {
            return None;
        }
        return Some(Self {
            id,
            name: name.to_string(),
        });
    }
}

#[test]
fn new_test() {
    let area = AreaEntity::new(1, "四万温泉");
    let inside: AreaEntity = area.expect("");
    assert!(inside.name == "四万温泉");
}

#[test]
#[should_panic]
fn new_test_none() {
    let area = AreaEntity::new(1, "");
    area.unwrap();
}
