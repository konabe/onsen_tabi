use crate::application::api_model::onsen_response::OnsenResponse;
use crate::domain::hotel_entity::HotelEntity;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HotelResponse {
    pub id: i32,
    pub name: String,
    pub has_washitsu: bool,
    pub solo_available: bool,
    pub url: String,
    pub description: String,
    pub onsens: Vec<OnsenResponse>,
}

impl From<HotelEntity> for HotelResponse {
    fn from(value: HotelEntity) -> Self {
        Self {
            id: value.id as i32,
            name: value.name.to_string(),
            has_washitsu: value.has_washitsu,
            solo_available: value.solo_available,
            url: value.url,
            description: value.description,
            onsens: value
                .onsens
                .iter()
                .map(|v| OnsenResponse::create(v.clone(), None))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HotelResponse;
    use crate::domain::hotel_entity::HotelEntity;

    #[test]
    fn test_hotel_response_from() {
        let hotel = HotelEntity::new(
            1,
            "ホテル",
            true,
            true,
            "https://example.com/hotel",
            "いい感じのホテル",
            &vec![],
        )
        .unwrap();
        let response = HotelResponse::from(hotel);
        assert_eq!(response.id, 1);
        assert_eq!(response.name, "ホテル");
        assert_eq!(response.has_washitsu, true);
        assert_eq!(response.solo_available, true);
        assert_eq!(response.url, "https://example.com/hotel");
        assert_eq!(response.description, "いい感じのホテル");
        assert_eq!(response.onsens.len(), 0);
    }
}
