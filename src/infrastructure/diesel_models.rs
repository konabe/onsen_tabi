use diesel::{Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name=crate::schema::hotel)]
pub struct Hotel {
    pub id: u32,
    pub name: String,
    pub has_washitsu: bool,
}
