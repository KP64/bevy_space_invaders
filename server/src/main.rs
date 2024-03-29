use crate::db::DB;
use anyhow::Result;
use rocket::{routes, serde::json::Json, State};
use utils::Entry;

mod db;

#[rocket::main]
async fn main() -> Result<()> {
    let db = DB::new().await?;

    let config = rocket::Config::figment().merge(("port", 3000));
    rocket::build()
        .configure(config)
        .manage(db)
        .mount("/", routes![get_scores, post_scores])
        .launch()
        .await?;

    Ok(())
}

#[rocket::get("/")]
async fn get_scores(db: &State<DB>) -> Json<Vec<Entry>> {
    let scores = db.select("scores").await.unwrap();
    Json(scores)
}

#[rocket::post("/", data = "<input>")]
async fn post_scores(db: &State<DB>, input: Json<Entry>) {
    db.create::<Vec<Entry>>("scores")
        .content(input.0)
        .await
        .unwrap();
}
