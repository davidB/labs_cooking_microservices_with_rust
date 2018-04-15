use super::schema::hello;

#[derive(Serialize, Debug, Queryable, Insertable)]
#[table_name = "hello"]
pub struct Hello {
    pub hello_id: i32,
    pub name: String,
}
