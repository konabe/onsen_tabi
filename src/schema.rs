// @generated automatically by Diesel CLI.

diesel::table! {
    hotel (id) {
        id -> Unsigned<Integer>,
        #[max_length = 255]
        name -> Varchar,
        has_washitsu -> Bool,
    }
}

diesel::table! {
    onsen (id) {
        id -> Unsigned<Integer>,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        spring_quality -> Varchar,
        #[max_length = 255]
        liquid -> Nullable<Varchar>,
        #[max_length = 255]
        osmotic_pressure -> Nullable<Varchar>,
        #[max_length = 255]
        category -> Varchar,
        hotel_id -> Nullable<Unsigned<Integer>>,
    }
}

diesel::joinable!(onsen -> hotel (hotel_id));

diesel::allow_tables_to_appear_in_same_query!(
    hotel,
    onsen,
);
