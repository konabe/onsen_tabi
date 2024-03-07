use crate::domain::onsen::onsen_entity::OnsenEntity;

#[derive(Clone)]
pub struct AreaEntity {
    pub id: u32,
    pub name: String,
    pub prefecture: String,
    pub national_resort: bool,
    pub village: Option<String>,
    pub url: String,
    pub description: String,
    pub onsens: Vec<OnsenEntity>,
}

impl AreaEntity {
    pub fn new(
        id: u32,
        name: &str,
        prefecture: &str,
        national_resort: bool,
        village: Option<&str>,
        url: &str,
        description: &str,
        onsens: Vec<OnsenEntity>,
    ) -> Option<Self> {
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
            national_resort,
            village: village.map(|v| v.to_string()),
            url: url.to_string(),
            description: description.to_string(),
            onsens,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::area_entity::AreaEntity;

    #[test]
    fn new_and_clone_test() {
        let area = AreaEntity::new(
            1,
            "四万",
            "群馬県",
            true,
            None,
            "https://nakanojo-kanko.jp/shima/",
            "",
            vec![],
        );
        let area: AreaEntity = area.expect("");
        assert!(area.name == "四万");
        let cloned_area = area.clone();
        assert!(cloned_area.name == "四万");
    }

    #[test]
    #[should_panic]
    fn new_test_return_none_if_name_is_empty() {
        let area = AreaEntity::new(
            1,
            "",
            "群馬県",
            false,
            None,
            "https://nakanojo-kanko.jp/shima/",
            "",
            vec![],
        );
        area.unwrap();
    }

    #[test]
    #[should_panic]
    fn new_test_return_none_if_prefecture_is_empty() {
        let area = AreaEntity::new(
            1,
            "四万",
            "",
            false,
            None,
            "https://nakanojo-kanko.jp/shima/",
            "",
            vec![],
        );
        area.unwrap();
    }
}
