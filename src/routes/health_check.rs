use actix_web::{get, HttpResponse};

/// Check that the service is still up
#[get("/health_check")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
