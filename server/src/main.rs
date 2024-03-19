use rocket::{routes, serde::json::Json};
use std::sync::OnceLock;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};
use utils::Entry;

static DB: OnceLock<Surreal<Client>> = OnceLock::new();

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = DB.get_or_init(Surreal::init);

    db.connect::<Ws>("127.0.0.1:8000").await?;
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    // TODO: Change NS & DB
    db.use_ns("test").use_db("test").await?;

    rocket::build()
        .mount("/", routes![get_scores, post_scores])
        .launch()
        .await?;

    Ok(())
}

#[rocket::get("/")]
async fn get_scores() -> Json<Vec<Entry>> {
    let db = DB.get().unwrap();
    let scores = db.select("scores").await.unwrap();
    Json(scores)
}

#[rocket::post("/", data = "<input>")]
async fn post_scores(input: Json<Entry>) {
    let db = DB.get().unwrap();
    db.create::<Vec<Entry>>("scores")
        .content(input.0)
        .await
        .unwrap();
}
