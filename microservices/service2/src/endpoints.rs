use rocket::http::Status;

use crate::repositories::EventRepository;
use crate::{DbConnection, models::Events};
use rocket::serde::json::serde_json::json;

#[get("/analytics?<limit>")]
pub async fn get_lekovi_analytics(
    connection: DbConnection,
    limit: i64,
) -> Result<rocket::serde::json::Value, Status> {
    connection
    .run(move |c| EventRepository::get_events_from_db(c, limit).map(|events: Vec<Events>| json!(events)))
        .await
        .map_err(|_| Status::NotFound)
}
