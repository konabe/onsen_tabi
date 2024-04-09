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
        access -> Text,
    }
}

diesel::table! {
    chemicals (id) {
        id -> Unsigned<Integer>,
        na_ion -> Unsigned<Integer>,
        ca_ion -> Unsigned<Integer>,
        mg_ion -> Unsigned<Integer>,
        cl_ion -> Unsigned<Integer>,
        hco3_ion -> Unsigned<Integer>,
        so4_ion -> Unsigned<Integer>,
        co2_ion -> Unsigned<Integer>,
        fe_ion -> Unsigned<Integer>,
        al_ion -> Unsigned<Integer>,
        cu_ion -> Unsigned<Integer>,
        h_ion -> Unsigned<Integer>,
        i_ion -> Unsigned<Integer>,
        s -> Unsigned<Integer>,
        rn -> Unsigned<Integer>,
        strong_na_cl -> Bool,
        #[max_length = 255]
        fe_type -> Varchar,
        weak_rn -> Bool,
    }
}

diesel::table! {
    hotel (id) {
        id -> Unsigned<Integer>,
        #[max_length = 255]
        name -> Varchar,
        has_washitsu -> Bool,
        solo_available -> Bool,
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
        temperature -> Nullable<Varchar>,
        #[max_length = 255]
        category -> Varchar,
        day_use -> Bool,
        #[max_length = 255]
        url -> Varchar,
        #[max_length = 255]
        img_url -> Nullable<Varchar>,
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
