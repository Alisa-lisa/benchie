
use serde_derive::{Deserialize, Serialize};


#[derive(Queryable, Debug)]
pub struct PastAddress {
    pub street: String,
    pub number: u16,
}

#[derive(Default, Serialize, Deserialize, Queryable, Debug)]
pub struct PastDestination {
    pub id: i32,
    pub passenger: i32,
    pub city: String,
    pub street: Option<String>,
    pub street_number: Option<String>,
    pub times_visited: Option<i32>,
}
