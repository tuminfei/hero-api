use rocket::http;
use rocket::request;
use rocket::Outcome;
use rocket::State;
use r2d2;
use r2d2_redis::RedisConnectionManager;
use std::ops::Deref;

use super::super::settings::Redis;


// Pool initiation.
// Call it starting an app and store a pul as a rocket managed state.
pub fn pool(database_config: Redis) -> Pool {
    let manager = RedisConnectionManager::new(&database_config.redis_address[..]).expect("connection manager");
    r2d2::Pool::new(manager).expect("db pool")
}

// Rocket guard type: a wrapper around an r2d2 pool.
// In conjunction with
// `impl<'a, 'r> request::FromRequest<'a, 'r> for RedisConnection` (see below)
// it allows code like:
//   ```
//   #[post("/<item>")]
//   fn create(item: &RawStr, connection: RedisConnection) -> ...
//
pub struct RedisConnection(pub r2d2::PooledConnection<RedisConnectionManager>);

// An alias to the type for a pool of redis connections.
type Pool = r2d2::Pool<RedisConnectionManager>;

// Retrieving a single connection from the managed database pool.
impl<'a, 'r> request::FromRequest<'a, 'r> for RedisConnection {
    type Error = ();

    fn from_request(request: &'a request::Request<'r>) -> request::Outcome<RedisConnection, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(RedisConnection(conn)),
            Err(_) => Outcome::Failure((http::Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for RedisConnection {
    type Target = r2d2::PooledConnection<RedisConnectionManager>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}