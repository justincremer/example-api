use crate::schema::users;
use diesel::prelude::*;

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub created_on: String,
    pub updated_on: String,
    pub email: String,
    pub first_name: String,
    pub last_name: Option<String>,
    password: String,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser<'s> {
    email: &'s str,
    first_name: &'s str,
    last_name: Option<&'s str>,
    password: &'s str,
}

pub fn all(connection: &PgConnection) -> Vec<User> {
    // use crate::schema::users::dsl::*;

    users::table
        .load(connection)
        .expect("failed to retrieve users")
}

// pub fn get(connection: &PgConnection, id: i32) -> User {
//     let res = users::table
//         .select(users::all_columns)
//         .filter(users::id.eq(id))
//         .load::<User>(connection)
//         .expect("Failed to retrieve user");

//     result::<User>(connection);
// }

// pub fn create(connection: &PgConnection, new_user: NewUser) -> User {
//     diesel::insert_into(users::table)
//         .values(&new_user)
//         .get_result(connection)
//         .expect("error creating new user")
// }
