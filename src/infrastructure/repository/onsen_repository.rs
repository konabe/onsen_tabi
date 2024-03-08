use crate::{
    domain::onsen::onsen_entity::OnsenEntity,
    infrastructure::mysql::{
        diesel_connection::establish_connection,
        diesel_model::{diesel_chemical::DieselChemical, diesel_onsen::Onsen, Sequence},
    },
    schema::{chemicals, onsen},
};
use diesel::*;

pub fn get_onsens(area_id: Option<u32>, hotel_id: Option<u32>) -> Vec<OnsenEntity> {
    let connection = &mut establish_connection();
    let mut query = onsen::table.into_boxed();
    if let Some(area_id) = area_id {
        query = query.filter(onsen::dsl::area_id.eq(area_id));
    }
    if let Some(hotel_id) = hotel_id {
        query = query.filter(onsen::dsl::hotel_id.eq(hotel_id));
    }
    let results: Vec<(Onsen, Option<DieselChemical>)> = query
        .left_join(chemicals::table)
        .select((Onsen::as_select(), Option::<DieselChemical>::as_select()))
        .load::<(Onsen, Option<DieselChemical>)>(connection)
        .expect("DB error");
    let onsen_entities = results
        .iter()
        .map(|v: &(Onsen, Option<DieselChemical>)| {
            let onsen_entity = OnsenEntity::create(v.0.clone(), v.1.clone());
            return onsen_entity;
        })
        .collect();
    onsen_entities
}

pub fn get_onsen(id: u32) -> Option<OnsenEntity> {
    let connection = &mut establish_connection();
    let results: Vec<(Onsen, Option<DieselChemical>)> = onsen::table
        .left_join(chemicals::table)
        .select((Onsen::as_select(), Option::<DieselChemical>::as_select()))
        .filter(onsen::dsl::id.eq(id))
        .load::<(Onsen, Option<DieselChemical>)>(connection)
        .expect("DB error");
    let result = results.first()?;
    Some(OnsenEntity::create(result.0.clone(), result.1.clone()))
}

pub fn put_onsen(onsen_entity: OnsenEntity) -> () {
    let updated_onsen = Onsen::from(onsen_entity.clone());
    let updated_chemicals = onsen_entity
        .clone()
        .quality
        .map(|v| DieselChemical::from(v));
    let connection = &mut establish_connection();
    let _ = connection.transaction(|connection| {
        let target_onsen_record: Vec<Onsen> = onsen::table
            .select(Onsen::as_select())
            .filter(onsen::dsl::id.eq(onsen_entity.id))
            .load::<Onsen>(connection)
            .expect("DB error");
        let chemical_id = target_onsen_record.first().and_then(|v| v.chemical_id);
        if let Some(current_chemical_id) = chemical_id {
            if let Some(updated_chemicals) = updated_chemicals.clone() {
                let _ = diesel::update(chemicals::table.find(current_chemical_id))
                    .set((
                        chemicals::dsl::na_ion.eq(updated_chemicals.na_ion),
                        chemicals::dsl::ca_ion.eq(updated_chemicals.ca_ion),
                        chemicals::dsl::mg_ion.eq(updated_chemicals.mg_ion),
                        chemicals::dsl::cl_ion.eq(updated_chemicals.cl_ion),
                        chemicals::dsl::hco3_ion.eq(updated_chemicals.hco3_ion),
                        chemicals::dsl::so4_ion.eq(updated_chemicals.so4_ion),
                        chemicals::dsl::co2_ion.eq(updated_chemicals.co2_ion),
                        chemicals::dsl::fe_ion.eq(updated_chemicals.fe_ion),
                        chemicals::dsl::al_ion.eq(updated_chemicals.al_ion),
                        chemicals::dsl::cu_ion.eq(updated_chemicals.cu_ion),
                        chemicals::dsl::h_ion.eq(updated_chemicals.h_ion),
                        chemicals::dsl::i_ion.eq(updated_chemicals.i_ion),
                        chemicals::dsl::s.eq(updated_chemicals.s),
                        chemicals::dsl::rn.eq(updated_chemicals.rn),
                        chemicals::dsl::strong_na_cl.eq(updated_chemicals.strong_na_cl),
                        chemicals::dsl::weak_rn.eq(updated_chemicals.weak_rn),
                    ))
                    .execute(connection)
                    .expect("DB error");
            } else {
                let _ = diesel::update(onsen::table.find(updated_onsen.id))
                    .set(onsen::dsl::chemical_id.eq(None::<u32>))
                    .execute(connection)
                    .expect("DB error");
                diesel::delete(chemicals::table.find(current_chemical_id))
                    .execute(connection)
                    .expect("DB error");
            }
        } else {
            if let Some(updated_chemicals) = updated_chemicals.clone() {
                diesel::insert_into(chemicals::table)
                    .values(updated_chemicals)
                    .execute(connection)
                    .expect("DB error");
                let new_chemical_id = Some(
                    diesel::sql_query("select LAST_INSERT_ID() as id")
                        .load::<Sequence>(connection)
                        .expect("get_id_error")
                        .first()
                        .unwrap()
                        .id as u32,
                );
                let _ = diesel::update(onsen::table.find(updated_onsen.id))
                    .set(onsen::dsl::chemical_id.eq(new_chemical_id))
                    .execute(connection)
                    .expect("DB error");
            }
        }
        let _ = diesel::update(onsen::table.find(updated_onsen.id))
            .set((
                onsen::dsl::name.eq(updated_onsen.name),
                onsen::dsl::spring_quality.eq(updated_onsen.spring_quality),
                onsen::dsl::liquid.eq(updated_onsen.liquid),
                onsen::dsl::osmotic_pressure.eq(updated_onsen.osmotic_pressure),
                onsen::dsl::temperature.eq(updated_onsen.temperature),
                onsen::dsl::category.eq(updated_onsen.category),
                onsen::dsl::day_use.eq(updated_onsen.day_use),
                onsen::dsl::url.eq(updated_onsen.url),
                onsen::dsl::img_url.eq(updated_onsen.img_url),
                onsen::dsl::description.eq(updated_onsen.description),
                onsen::dsl::hotel_id.eq(updated_onsen.hotel_id),
            ))
            .execute(connection)
            .expect("DB error");

        diesel::result::QueryResult::Ok(())
    });
}

pub fn post_onsen(onsen_entity: OnsenEntity) -> OnsenEntity {
    let mut new_onsen = Onsen::from(onsen_entity.clone());
    let new_chemicals = onsen_entity
        .clone()
        .quality
        .map(|v| DieselChemical::from(v));
    let connection = &mut establish_connection();
    let _ = connection.transaction(|connection| {
        let mut generated_id: Option<u32> = None;
        if let Some(new_chemicals) = new_chemicals.clone() {
            diesel::insert_into(chemicals::table)
                .values(&new_chemicals)
                .execute(connection)
                .expect("DB error");

            generated_id = Some(
                diesel::sql_query("select LAST_INSERT_ID() as id")
                    .load::<Sequence>(connection)
                    .expect("get_id_error")
                    .first()
                    .unwrap()
                    .id as u32,
            );
        }
        new_onsen.chemical_id = generated_id;
        diesel::insert_into(onsen::table)
            .values(&new_onsen)
            .execute(connection)
            .expect("DB error");

        diesel::result::QueryResult::Ok(())
    });
    OnsenEntity::create(new_onsen, new_chemicals)
}
