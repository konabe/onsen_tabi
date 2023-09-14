use super::super::mysql::diesel_connection::establish_connection;
use crate::{infrastructure::mysql::diesel_model::diesel_user::User, schema::user};
use diesel::*;

pub fn exists_user(email: &str) -> bool {
    let connection = &mut establish_connection();
    let results: Vec<User> = user::table
        .select(User::as_select())
        .filter(user::dsl::email.eq(email))
        .load(connection)
        .expect("DB Error");
    !results.is_empty()
}

pub fn get_user(email: &str) -> Option<User> {
    let connection = &mut establish_connection();
    let results: Vec<User> = user::table
        .select(User::as_select())
        .filter(user::dsl::email.eq(email))
        .load(connection)
        .expect("DB Error");
    results.first().map(|v| v.clone())
}

pub fn post_user(email: &str, hashed_password: &str) {
    let new_user = User {
        id: 0,
        email: email.to_string(),
        hashed_password: hashed_password.to_string(),
        role: "user".to_string(),
    };
    let connection = &mut establish_connection();
    diesel::insert_into(user::table)
        .values(&new_user)
        .execute(connection)
        .expect("DB error");
}
