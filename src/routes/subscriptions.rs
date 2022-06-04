use actix_web::{post, web, HttpResponse};
use sqlx::PgConnection;

use chrono::Utc;
use uuid::Uuid;

#[derive(serde::Deserialize)]
struct SubscribeForm {
    name: String,
    email: String,
}

/// Handle new subscriptions
#[post("/subscriptions")]
async fn subscribe(
    form: web::Form<SubscribeForm>,
    connection: web::Data<PgConnection>,
) -> HttpResponse {
    sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)"#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    // We use `get_ref` to get an immutable reference to the `PgConnection`
    // wrapped by `web::Data`.
    .execute(connection.get_ref())
    .await;
    HttpResponse::Ok().finish()
}
