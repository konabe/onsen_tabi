use crate::domain::area_entity::AreaEntity;
use diesel::{Identifiable, Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable, Identifiable, Insertable, Debug, Clone)]
#[diesel(table_name=crate::schema::area)]
pub struct Area {
    pub id: u32,
    pub name: String,
    pub prefecture: String,
    pub national_resort: bool,
    pub village: Option<String>,
    pub url: String,
    pub description: String,
    pub access: String,
}

impl From<Area> for AreaEntity {
    fn from(value: Area) -> Self {
        AreaEntity::new(
            value.id,
            &value.name,
            &value.prefecture,
            value.national_resort,
            value.village.as_deref(),
            &value.url,
            &value.description,
            &value.access,
            vec![],
        )
        .expect("Saved data violates AreaEntity")
    }
}

impl From<AreaEntity> for Area {
    fn from(value: AreaEntity) -> Self {
        Self {
            id: value.id,
            name: value.name,
            prefecture: value.prefecture,
            national_resort: value.national_resort,
            village: value.village,
            url: value.url,
            description: value.description,
            access: value.access,
        }
    }
}
