use diesel::*;

use super::mysql::{diesel_connection::establish_connection, diesel_models::User};
use crate::schema::user;

pub fn post_user(email: String, hashed_password: String, salt: String) {
    let new_user = User {
        id: 0,
        email,
        hashed_password,
        salt,
        role: "user".to_string(),
    };
    let connection = &mut establish_connection();
    diesel::insert_into(user::table)
        .values(&new_user)
        .execute(connection)
        .expect("DB error");
}
