use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::AsyncPgConnection;
use std::env;

pub type DbPool = Pool<AsyncPgConnection>;

pub fn establish_connection_pool() -> DbPool {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);

    Pool::builder(config)
        .build()
        .expect("Failed to create pool")
}
