use crate::{establish_connection, schema::schema::users};
use diesel::{
    deserialize::Queryable, prelude::Insertable, RunQueryDsl, Selectable, SelectableHelper,
};

#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub created_at: String,
    pub updated_at: String,
}

impl User {
    pub fn new(
        id: i32,
        username: String,
        full_name: String,
        email: String,
        password: String,
        created_at: String,
        updated_at: String,
    ) -> Self {
        Self {
            id,
            username,
            full_name,
            email,
            password,
            created_at,
            updated_at,
        }
    }

    pub fn add_user(user: User) {
        let connection = &mut establish_connection();

        diesel::insert_into(users::table)
            .values(&user)
            .returning(User::as_returning())
            .get_result(connection)
            .expect("Error saving new post");
    }
}
