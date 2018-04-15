use super::schema::reviews;

#[derive(Serialize, Debug, Queryable, Insertable)]
#[table_name = "reviews"]
pub struct Review {
    pub product_id: i32,
    pub reviewer: String,
    pub review: String,
}
