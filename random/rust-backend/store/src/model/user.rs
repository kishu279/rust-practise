use crate::store::Store;
use diesel::{prelude::*, };
use uuid::Uuid;

#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct User {
    id: Uuid,
    name: String,
    email: String,
}

impl Store {
    pub fn create_user(&mut self, name: String, email: String) -> Result<String, diesel::result::Error> {
        let id = Uuid::new_v4();

        let u = User {
            id: id,
            name: name,
            email: email,
        };

        diesel::insert_into(crate::schema::users::table)
                    .values(&u)
                    .returning(User::as_returning())
                    .get_result(&mut self.conn)?;    
    
        Ok(id.to_string())
    }
}




