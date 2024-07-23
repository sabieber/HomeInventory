use diesel::data_types::PgTimestamp;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::locations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Location {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_at: PgTimestamp,
    pub updated_at: PgTimestamp,
}
