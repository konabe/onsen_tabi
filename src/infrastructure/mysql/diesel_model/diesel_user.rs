use diesel::{Identifiable, Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable, Identifiable, Insertable, Debug, Clone)]
#[diesel(table_name=crate::schema::user)]
pub struct User {
    pub id: u32,
    pub email: String,
    pub hashed_password: String,
    pub role: String,
}
