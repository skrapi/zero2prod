use actix_web::{post, web, HttpResponse};
use sqlx::PgPool;

use chrono::Utc;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
struct SubscribeForm {
    name: String,
    email: String,
}

/// Handle new subscriptions
#[post("/subscriptions")]
async fn subscribe(form: web::Form<SubscribeForm>, db_pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();

    // % tells tracing to use `Display` implementation in logging
    let request_span = tracing::info_span!("Adding a new subscriber.", %request_id, subscriber_email = %form.email, subscriber_name= %form.name);
    let _request_span_guard = request_span.enter();

    // We do not call `.enter` on query_span!
    // `.instrument` takes care of it at the right moments
    // in the quirey future lifetime
    let query_span = tracing::info_span!("Saving new subscriber details in the database.");

    match sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)"#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    // We use `get_ref` to get an immutable reference to the `PgPool`
    // wrapped by `web::Data`.
    .execute(db_pool.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            // This error log falls outside of the `query_span`
            // We will rectify that later
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
