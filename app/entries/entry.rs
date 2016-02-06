#[derive(Queryable)]
use diesel::types::Timestamp;

pub struct Entry {
  pub id: i32,
  pub title: String,
  pub body: String,
  pub public: bool,
  pub created_at: Timestamp
}