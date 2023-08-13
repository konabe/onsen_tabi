use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable, Identifiable, Insertable, Debug)]
#[diesel(table_name=crate::schema::hotel)]
pub struct Hotel {
    pub id: u32,
    pub name: String,
    pub has_washitsu: bool,
}

#[derive(Queryable, Selectable, Identifiable, Insertable, Associations, Debug)]
#[diesel(belongs_to(Hotel))]
#[diesel(table_name=crate::schema::onsen)]
pub struct Onsen {
    pub id: u32,
    pub name: String,
    pub spring_quality: String,
    pub liquid: Option<String>,
    pub osmotic_pressure: Option<String>,
    pub category: String,
    pub description: String,
    pub hotel_id: Option<u32>,
}

#[derive(Queryable, Selectable, Identifiable, Insertable, Debug)]
#[diesel(table_name=crate::schema::area)]
pub struct Area {
    pub id: u32,
    pub name: String,
    pub prefecture: String,
}
