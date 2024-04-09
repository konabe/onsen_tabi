use crate::domain::area_entity::AreaEntity;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AreaRequest {
    pub name: String,
    pub prefecture: String,
    pub national_resort: bool,
    pub village: Option<String>,
    pub url: String,
    pub description: String,
    pub access: String,
}

impl AreaRequest {
    pub fn create_entity(&self, id: u32) -> Option<AreaEntity> {
        AreaEntity::new(
            id,
            self.name.as_str(),
            self.prefecture.as_str(),
            self.national_resort,
            self.village.as_deref(),
            self.url.as_str(),
            self.description.as_str(),
            self.access.as_str(),
            vec![],
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::application::api_model::area_request::AreaRequest;
    use crate::domain::area_entity::AreaEntity;

    #[test]
    fn test_create_entity() {
        let area_request = AreaRequest {
            name: "四万".to_string(),
            prefecture: "群馬県".to_string(),
            national_resort: true,
            village: None,
            url: "https://nakanojo-kanko.jp/shima/".to_string(),
            description: "".to_string(),
            access: "".to_string(),
        };
        let area_entity: AreaEntity = area_request.create_entity(1).unwrap();
        assert_eq!(area_entity.id, 1);
        assert_eq!(area_entity.name, "四万");
        assert_eq!(area_entity.prefecture, "群馬県");
        assert_eq!(area_entity.national_resort, true);
        assert_eq!(area_entity.village, None);
        assert_eq!(area_entity.url, "https://nakanojo-kanko.jp/shima/");
        assert_eq!(area_entity.description, "");
        assert_eq!(area_entity.access, "");
        assert_eq!(area_entity.onsens.len(), 0);
    }
}
