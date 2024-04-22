use sqlx::{MySql, Pool};

pub struct ServerState {
    pub pool: Pool<MySql>,
}
