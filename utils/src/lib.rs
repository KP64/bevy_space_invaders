use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Entry {
    pub name: String,
    pub score: i64,
}

#[cfg(test)]
mod tests {}
