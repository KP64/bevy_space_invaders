use std::ops;

use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Result, Surreal,
};

const IP: &str = "127.0.0.1";
const PORT: usize = 8000;

#[derive(Clone)]
pub struct DB(pub Surreal<Client>);

impl ops::Deref for DB {
    type Target = Surreal<Client>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DB {
    pub async fn new() -> Result<Self> {
        let db = Surreal::new::<Ws>(format!("{IP}:{PORT}")).await?;

        db.signin(Root {
            username: "root",
            password: "root",
        })
        .await?;

        db.use_ns("test").use_db("test").await?;
        Ok(Self(db))
    }
}
