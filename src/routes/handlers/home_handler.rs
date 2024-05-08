use actix_web::{get, web, Responder};
use sea_orm::{ConnectionTrait, Statement};

use crate::utils::{api_response::{self, ApiResponse}, app_state::{self, AppState}};



#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    
    api_response::ApiResponse::new(200, format!("Hello {name}!"))
}



#[get("/test")]
pub async fn test(
    app_state: web::Data<AppState>
) -> Result<ApiResponse,ApiResponse> {

    let res = app_state.db
    .query_all(Statement::from_string(sea_orm::DatabaseBackend::Postgres, "Select * from user; "))
    .await
    .map_err(|err| ApiResponse::new(500,err.to_string()))?;

    Ok(api_response::ApiResponse::new(200, "Test".to_string()))
}
