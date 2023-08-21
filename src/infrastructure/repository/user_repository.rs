use super::super::mysql::{diesel_connection::establish_connection, diesel_models::User};
use crate::schema::user;
use diesel::*;

pub fn exists_user(email: String) -> bool {
    let connection = &mut establish_connection();
    let results: Vec<User> = user::table
        .select(User::as_select())
        .filter(user::dsl::email.eq(email))
        .load(connection)
        .expect("DB Error");
    !results.is_empty()
}

pub fn get_user(email: String) -> Option<User> {
    let connection = &mut establish_connection();
    let results: Vec<User> = user::table
        .select(User::as_select())
        .filter(user::dsl::email.eq(email))
        .load(connection)
        .expect("DB Error");
    results.first().map(|v| v.clone())
}

pub fn post_user(email: String, hashed_password: String) {
    let new_user = User {
        id: 0,
        email,
        hashed_password,
        role: "user".to_string(),
    };
    let connection = &mut establish_connection();
    diesel::insert_into(user::table)
        .values(&new_user)
        .execute(connection)
        .expect("DB error");
}