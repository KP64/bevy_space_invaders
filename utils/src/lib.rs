use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    pub name: String,
    pub score: usize,
}

#[cfg(test)]
mod tests {}
