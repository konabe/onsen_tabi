#[derive(Clone)]
pub struct AreaEntity {
    pub id: u32,
    pub name: String,
    pub prefecture: String,
}

impl AreaEntity {
    pub fn new(id: u32, name: &str, prefecture: &str) -> Option<Self> {
        if name.is_empty() {
            return None;
        }
        if prefecture.is_empty() {
            return None;
        }
        Some(Self {
            id,
            name: name.to_string(),
            prefecture: prefecture.to_string(),
        })
    }
}

#[test]
fn new_test() {
    let area = AreaEntity::new(1, "四万", "群馬県");
    let inside: AreaEntity = area.expect("");
    assert!(inside.name == "四万");
}

#[test]
#[should_panic]
fn new_test_none_name() {
    let area = AreaEntity::new(1, "", "群馬県");
    area.unwrap();
}

#[test]
#[should_panic]
fn new_test_none_prefecture() {
    let area = AreaEntity::new(1, "四万", "");
    area.unwrap();
}
