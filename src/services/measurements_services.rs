use diesel::prelude::*;
use diesel::PgConnection;
use uuid::Uuid;

use crate::models::measurement::{Measurement, NewMeasurement};

pub fn get_measurements(conn: &PgConnection) -> Vec<Measurement> {
    use crate::schema::measurements::dsl::*;
    measurements
        .load::<Measurement>(conn)
        .expect("No measurements")
}

pub fn create_measurement(name: &str, short_name: &str, conn: &PgConnection) -> Measurement {
    use crate::schema::measurements;
    let id = Uuid::new_v4().to_string();
    let new: NewMeasurement = NewMeasurement {
        id: &id,
        name,
        short_name,
    };
    diesel::insert_into(measurements::table)
        .values(&new)
        .get_result(conn)
        .expect("Error saving NewMeasurement")
}

pub fn get_measurement(
    measurement_id: &str,
    conn: &PgConnection,
) -> Result<Measurement, &'static str> {
    use crate::schema::measurements::dsl::*;
    let result = measurements
        .filter(id.eq(measurement_id))
        .load::<Measurement>(conn)
        .expect("Error loading measurement");
    Ok(Measurement {
        id: result[0].id.clone(),
        name: result[0].name.clone(),
        short_name: result[0].short_name.clone(),
    })
}
