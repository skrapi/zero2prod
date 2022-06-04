use actix_web::{post, web, HttpResponse};

#[derive(serde::Deserialize)]
struct SubscribeForm {
    name: String,
    email: String,
}

/// Handle new subscriptions
#[post("/subscriptions")]
async fn subscribe(_form: web::Form<SubscribeForm>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
