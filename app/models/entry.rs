#[derive(Queryable)]
pub struct Entry {
  pub id: i32,
  pub title: String,
  pub body: String,
  pub public: bool
}
