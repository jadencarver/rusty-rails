use diesel::pg::PgConnection;
pub fn establish_connection() -> PgConnection {
    use diesel::connection::Connection;
    use dotenv::dotenv;
    use std::env;
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
