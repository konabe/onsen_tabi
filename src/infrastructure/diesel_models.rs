use diesel::{Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(table_name=crate::schema::hotel)]
pub struct Hotel {
    pub id: u32,
    pub name: String,
    pub has_washitsu: bool,
}
