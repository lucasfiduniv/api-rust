use actix_web::{
    web::{
        scope,
        ServiceConfig
    },
    get,
    HttpResponse,
    Responder
};

use serde_json::json;

#[get("/healthchecker")]
async fn health_checker() -> impl Responder {
    const MESSAGE: &str = "health check API is runining";
    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": MESSAGE
    }))
}
pub fn config(conf: &mut ServiceConfig) {
    let scope = scope("/api").service(health_checker);

    conf.service(scope);
}