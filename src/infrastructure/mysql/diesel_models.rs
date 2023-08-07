use diesel::{Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name=crate::schema::hotel)]
pub struct Hotel {
    pub id: u32,
    pub name: String,
    pub has_washitsu: bool,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name=crate::schema::onsen)]
pub struct Onsen {
    pub id: u32,
    pub name: String,
    pub spring_quality: String,
    pub liquid: Option<String>,
    pub osmotic_pressure: Option<String>,
    pub category: String,
    pub hotel_id: Option<u32>,
}
