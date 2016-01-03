extern crate mysql;

use mysql::conn::MyOpts;
use mysql::conn::pool::MyPool;
use mysql::value::from_row;

let opts = MyOpts {
  user: Some("root".to_string()),
  pass: None,
  db_name: Some("rusty_rails_dev".to_string()),
  ..Default::default()
};
let pool = MyPool::new(opts).unwrap();
pool.prep_exec(r"DROP TABLE IF EXISTS requests", ()).unwrap();
pool.prep_exec(r"CREATE TABLE requests (
                   customer_id int not null
                )", ()).unwrap();
