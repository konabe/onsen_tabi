// @generated automatically by Diesel CLI.

diesel::table! {
    area (id) {
        id -> Unsigned<Integer>,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        prefecture -> Varchar,
        national_resort -> Bool,
        #[max_length = 255]
        village -> Nullable<Varchar>,
        #[max_length = 255]
        url -> Varchar,
        description -> Text,
    }
}

diesel::table! {
    chemicals (id) {
        id -> Unsigned<Integer>,
        na_ion -> Bool,
        ca_ion -> Bool,
        mg_ion -> Bool,
        cl_ion -> Bool,
        hco3_ion -> Bool,
        so4_ion -> Bool,
        co2_ion -> Bool,
        fe_ion -> Bool,
        h_ion -> Bool,
        i_ion -> Bool,
        s -> Bool,
        rn -> Bool,
    }
}

diesel::table! {
    hotel (id) {
        id -> Unsigned<Integer>,
        #[max_length = 255]
        name -> Varchar,
        has_washitsu -> Bool,
        #[max_length = 255]
        url -> Varchar,
        description -> Text,
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
        day_use -> Bool,
        #[max_length = 255]
        url -> Varchar,
        description -> Text,
        chemical_id -> Nullable<Unsigned<Integer>>,
        hotel_id -> Nullable<Unsigned<Integer>>,
        area_id -> Nullable<Unsigned<Integer>>,
    }
}

diesel::table! {
    user (id) {
        id -> Unsigned<Integer>,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        hashed_password -> Varchar,
        #[max_length = 255]
        role -> Varchar,
    }
}

diesel::joinable!(hotel -> area (area_id));
diesel::joinable!(onsen -> area (area_id));
diesel::joinable!(onsen -> chemicals (chemical_id));
diesel::joinable!(onsen -> hotel (hotel_id));

diesel::allow_tables_to_appear_in_same_query!(
    area,
    chemicals,
    hotel,
    onsen,
    user,
);
