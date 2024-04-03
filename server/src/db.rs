use anyhow::Result;
use std::ops;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

const IP: &str = "127.0.0.1"; // TODO: Try "localhost", "0.0.0.0"
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
        let address = format!("{IP}:{PORT}");
        let db = Surreal::new::<Ws>(&address)
            .await
            .expect(&format!("No Connection to Address {address}"));

        db.signin(Root {
            username: &dotenvy::var("DB_USER")?,
            password: &dotenvy::var("DB_PASSWORD")?,
        })
        .await
        .expect("Could not sign in to db");

        db.use_ns("test").use_db("test").await?;
        Ok(Self(db))
    }
}
