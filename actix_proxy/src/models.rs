use diesel::deserialize::Queryable;

#[derive(Queryable)]
pub struct PastAddress {
    pub street: String,
    pub number: u16,
}
