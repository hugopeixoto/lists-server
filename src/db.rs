use diesel::r2d2;

pub type Connection = diesel::PgConnection;

pub type ConnectionManager = r2d2::ConnectionManager<Connection>;
pub type Pool = r2d2::Pool<ConnectionManager>;

pub type Result<T> = core::result::Result<T, diesel::result::Error>;
