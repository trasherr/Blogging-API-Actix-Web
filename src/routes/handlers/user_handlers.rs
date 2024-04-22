use actix_web::{get, web, Responder};

use crate::utils::{api_response, app_state};



#[get("")]
pub async fn user(
    app_state: web::Data<app_state::AppState>
) -> impl Responder {

    api_response::ApiResponse::new(200, "Verifyed user".to_string())
}