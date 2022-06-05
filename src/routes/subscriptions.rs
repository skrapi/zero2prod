use actix_web::{post, web, HttpResponse};
use sqlx::PgPool;

use chrono::Utc;
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
    log::info!(
        "request_id {} - Adding a new subscriber: '{}' '{}'",
        request_id,
        form.email,
        form.name
    );
    log::info!(
        "request_id {} - Saving new subscriber details in the database.",
        request_id
    );
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
    .await
    {
        Ok(_) => {
            log::info!(
                "request_id {} - New subscriber details have been saved.",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!(
                "request_id {} - Failed to execute query: {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
