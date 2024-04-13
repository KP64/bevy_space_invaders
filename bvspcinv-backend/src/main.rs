use rocket::{routes, serde::json::Json, State};
use shuttle_runtime::CustomError;
use sqlx::{Executor, PgPool};
use utils::Entry;

pub struct DB {
    pool: PgPool,
}

#[shuttle_runtime::main]
async fn rocket(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_rocket::ShuttleRocket {
    pool.execute(include_str!("../schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let state = DB { pool };
    let rocket = rocket::build()
        .mount("/", routes![get_scores, post_scores])
        .manage(state);

    Ok(rocket.into())
}

#[rocket::get("/")]
async fn get_scores(db: &State<DB>) -> Result<Json<Vec<Entry>>, String> {
    let res = sqlx::query_as::<_, Entry>("SELECT * FROM entries;")
        .fetch_all(&db.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(Json(res))
}

#[rocket::post("/", data = "<input>")]
async fn post_scores(db: &State<DB>, input: Json<Entry>) -> Result<(), String> {
    sqlx::query("INSERT INTO entries (name, score) VALUES ($1, $2) ON CONFLICT (name) DO UPDATE SET name = $1, score = $2;")
        .bind(&input.name)
        .bind(input.score)
        .execute(&db.pool)
        .await
        .map_err(|e| format!("Couldn't Insert or Update Entry: {e}"))?;

    Ok(())
}
