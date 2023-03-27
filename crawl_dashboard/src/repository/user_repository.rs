use crate::db::schema::users::dsl::users;
use diesel::{prelude::*, PgConnection};

use crate::{
    db::{
        models::{NewUser, User},
        schema::users as users_schema,
    },
    errors::api_error::ApiError,
};

pub fn create_user(conn: &mut PgConnection, new_user: &NewUser) -> Result<User, ApiError> {
    Ok(diesel::insert_into(users_schema::table)
        .values(new_user)
        .get_result(conn)?)
}

pub fn get_all_users(conn: &mut PgConnection) -> Result<Vec<User>, ApiError> {
    Ok(users.load::<User>(conn)?)
}

pub fn get_user(conn: &mut PgConnection, id: i32) -> Result<User, ApiError> {
    let user = users.find(id).first::<User>(conn)?;
    Ok(user)
}
