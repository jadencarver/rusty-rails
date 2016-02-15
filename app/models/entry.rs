#[derive(Queryable)]
pub struct Entry {
  pub id: i32,
  pub title: String,
  pub body: String,
  pub public: bool
}

impl Entry {
    pub fn blank() -> Entry {
        Entry {
            id: 0,
            title: "Hello!".to_string(),
            body: "".to_string(),
            public: false
        }
    }
}
