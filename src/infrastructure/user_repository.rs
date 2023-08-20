use diesel::*;

use super::mysql::{diesel_connection::establish_connection, diesel_models::User};
use crate::schema::user;

pub fn exists_user(email: String) -> bool {
    let connection = &mut establish_connection();
    let results: Vec<User> = user::table
        .select(User::as_select())
        .filter(user::dsl::email.eq(email))
        .load(connection)
        .expect("DB Error");
    !results.is_empty()
}

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
