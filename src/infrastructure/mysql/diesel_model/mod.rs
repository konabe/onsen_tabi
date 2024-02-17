pub mod diesel_area;
pub mod diesel_chemical;
pub mod diesel_hotel;
pub mod diesel_onsen;
pub mod diesel_user;

use diesel::{sql_types::Bigint, QueryableByName};

#[derive(QueryableByName)]
pub struct Sequence {
    #[diesel(sql_type = Bigint)]
    pub id: i64,
}
