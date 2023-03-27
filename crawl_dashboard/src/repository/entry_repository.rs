use diesel::prelude::*;
use diesel::PgConnection;

use crate::db::models::*;
use crate::db::schema::*;
use crate::errors::api_error::ApiError;

pub fn create_entry(conn: &mut PgConnection, new_entry: &NewEntry) -> Result<Entry, ApiError> {
    Ok(diesel::insert_into(entries::table)
        .values(new_entry)
        .get_result(conn)?)
}

pub fn get_all_entries_for_user(
    conn: &mut PgConnection,
    user_id: i32,
) -> Result<Vec<Entry>, ApiError> {
    let user = users::dsl::users.find(user_id).first::<User>(conn)?;

    let e = Entry::belonging_to(&user)
        .select(Entry::as_select())
        .load(conn)?;
    
    Ok(e)
}
