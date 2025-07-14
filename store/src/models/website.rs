use crate::store::Store;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::website)]
#[diesel(check_for_backend(diesel::pg::Pg))]


pub struct Website {
    pub id: String,
    pub url: String,
    pub user_id: String,
    pub time_added: chrono::NaiveDateTime,
}

impl Store {
    pub fn create_website(&mut self, url: String, user_id: String) -> Result<Website, diesel::result::Error>{
        let w = Website {
            id: Uuid::new_v4().to_string(),
            url,    
            user_id,
            time_added: chrono::Utc::now().naive_utc(),
        };
        diesel::insert_into(crate::schema::website::table)
            .values(&w)
            .returning(Website::as_returning())
            .get_result(&mut self.conn)
            .expect("Error saving new website");

        Ok(w)
    }
    pub fn get_website(&mut self, input_id: String) -> Result<Website, diesel::result::Error>{
        use crate::schema::website::dsl::*;
        let website_result = website.filter(id.eq(input_id))
            .select(Website::as_select())
            .first(&mut self.conn)?;
        
        Ok(website_result)
    }
}