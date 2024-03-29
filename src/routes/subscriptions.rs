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
/// % tells tracing to use `Display` implementation in logging
#[tracing::instrument (
    name = "Adding a new subscriber.",
    skip(form, pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
#[post("/subscriptions")]
async fn subscribe(form: web::Form<SubscribeForm>, pool: web::Data<PgPool>) -> HttpResponse {
    match insert_subscriber(&pool, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = " Saving new subscriber details in the database.",
    skip(form, pool)
)]
async fn insert_subscriber(pool: &PgPool, form: &SubscribeForm) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)"#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
