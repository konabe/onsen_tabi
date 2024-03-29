use crate::domain::hotel_entity::HotelEntity;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HotelRequest {
    pub name: String,
    pub has_washitsu: bool,
    pub solo_available: bool,
    pub url: String,
    pub description: String,
}

impl HotelRequest {
    pub fn create_entity(&self, id: u32) -> Option<HotelEntity> {
        HotelEntity::new(
            id,
            self.name.as_str(),
            self.has_washitsu,
            self.solo_available,
            self.url.as_str(),
            self.description.as_str(),
            &vec![],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::HotelRequest;

    #[test]
    fn test_hotel_request_create_entity() {
        let request = HotelRequest {
            name: "ホテル".to_string(),
            has_washitsu: true,
            solo_available: true,
            url: "https://example.com/hotel".to_string(),
            description: "いい感じのホテル".to_string(),
        };
        let entity = request.create_entity(1).unwrap();
        assert_eq!(entity.id, 1);
        assert_eq!(entity.name, "ホテル");
        assert_eq!(entity.has_washitsu, true);
        assert_eq!(entity.solo_available, true);
        assert_eq!(entity.url, "https://example.com/hotel");
        assert_eq!(entity.description, "いい感じのホテル");
    }
}
