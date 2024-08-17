use crate::domain::onsen::onsen_entity::OnsenEntity;

#[derive(Clone)]
pub struct AreaEntity {
    pub id: u32,
    pub name: String,
    pub kana: String,
    pub prefecture: String,
    pub national_resort: bool,
    pub village: Option<String>,
    pub url: String,
    pub description: String,
    pub access: String,
    pub onsens: Vec<OnsenEntity>,
}

#[derive(Clone, Default)]
pub struct AreaEntityBuilder {
    pub id: u32,
    pub name: String,
    pub kana: String,
    pub prefecture: String,
    pub national_resort: bool,
    pub village: Option<String>,
    pub url: String,
    pub description: String,
    pub access: String,
    pub onsens: Vec<OnsenEntity>,
}

impl AreaEntityBuilder {
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

    pub fn kana(mut self, kana: &str) -> Self {
        self.kana = kana.to_string();
        self
    }

    pub fn prefecture(mut self, prefecture: &str) -> Self {
        self.prefecture = prefecture.to_string();
        self
    }

    pub fn national_resort(mut self, national_resort: bool) -> Self {
        self.national_resort = national_resort;
        self
    }

    pub fn village(mut self, village: Option<&str>) -> Self {
        self.village = village.map(|v| v.to_string());
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

    pub fn access(mut self, access: &str) -> Self {
        self.access = access.to_string();
        self
    }

    pub fn onsens(mut self, onsens: Vec<OnsenEntity>) -> Self {
        self.onsens = onsens;
        self
    }

    pub fn build(self) -> Option<AreaEntity> {
        if self.name.is_empty() {
            return None;
        }
        if self.prefecture.is_empty() {
            return None;
        }
        Some(AreaEntity {
            id: self.id,
            name: self.name,
            kana: self.kana,
            prefecture: self.prefecture,
            national_resort: self.national_resort,
            village: self.village,
            url: self.url,
            description: self.description,
            access: self.access,
            onsens: self.onsens,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::area_entity::AreaEntityBuilder;

    #[test]
    fn new_and_clone_test() {
        let area = AreaEntityBuilder::new()
            .id(1)
            .name("四万")
            .kana("しま")
            .prefecture("群馬県")
            .national_resort(true)
            .village(None)
            .url("https://nakanojo-kanko.jp/shima/")
            .description("")
            .access("")
            .onsens(vec![])
            .build()
            .expect("");
        assert!(area.name == "四万");
        let cloned_area = area.clone();
        assert!(cloned_area.name == "四万");
    }

    #[test]
    #[should_panic]
    fn new_test_return_none_if_name_is_empty() {
        let area = AreaEntityBuilder::new()
            .id(1)
            .name("")
            .kana("しま")
            .prefecture("群馬県")
            .national_resort(false)
            .village(None)
            .url("https://nakanojo-kanko.jp/shima/")
            .description("")
            .access("")
            .onsens(vec![])
            .build();
        area.unwrap();
    }

    #[test]
    #[should_panic]
    fn new_test_return_none_if_prefecture_is_empty() {
        let area = AreaEntityBuilder::new()
            .id(1)
            .name("四万")
            .kana("しま")
            .prefecture("")
            .national_resort(false)
            .village(None)
            .url("https://nakanojo-kanko.jp/shima/")
            .description("")
            .access("")
            .onsens(vec![])
            .build();
        area.unwrap();
    }
}
