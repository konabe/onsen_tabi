// @generated automatically by Diesel CLI.

diesel::table! {
    area (id) {
        id -> Unsigned<Integer>,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    hotel (id) {
        id -> Unsigned<Integer>,
        #[max_length = 255]
        name -> Varchar,
        has_washitsu -> Bool,
        area_id -> Nullable<Unsigned<Integer>>,
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
        area_id -> Nullable<Unsigned<Integer>>,
    }
}

diesel::joinable!(hotel -> area (area_id));
diesel::joinable!(onsen -> area (area_id));

diesel::allow_tables_to_appear_in_same_query!(
    area,
    hotel,
    onsen,
);
