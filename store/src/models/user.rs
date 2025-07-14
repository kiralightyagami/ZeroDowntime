use crate::store::Store;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]

struct User {
    id: String,
    username: String,
    password: String,
}

impl Store {
    pub fn sign_up(&mut self, username: String, password: String) -> Result<String, diesel::result::Error>{
        let u = User {
            id: Uuid::new_v4().to_string(),
            username,
            password,
        };
        diesel::insert_into(crate::schema::user::table)
            .values(&u)
            .returning(User::as_returning())
            .get_result(&mut self.conn)
            .expect("Error saving new user");

        Ok(u.id)
    }

    pub fn sign_in(&mut self, input_username: String, input_password: String) -> Result<bool, diesel::result::Error>{
        use crate::schema::user::dsl::*;
        let user_result = user
        .filter(username.eq(input_username))
        .select(User::as_select())
        .first(&mut self.conn)
        .expect("user not found");
        
        if user_result.password != input_password{
            return Ok(false);
        }
        Ok(true)
    }


}