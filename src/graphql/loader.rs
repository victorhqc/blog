use crate::db::SeaOrmPool;

pub struct DataLoader {
    pub pool: SeaOrmPool,
}

impl DataLoader {
    pub fn new(pool: &SeaOrmPool) -> Self {
        Self { pool: pool.clone() }
    }
}
