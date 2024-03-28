use actix_web::{get, web, Responder};
use sea_orm::{ConnectionTrait, Statement};

use crate::utils::{api_response, app_state::{self, AppState}};



#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    
    api_response::ApiResponse::new(200, format!("Hello {name}!"))
}



#[get("/test")]
pub async fn test(app_state: web::Data<AppState>) -> impl Responder {

    let res = app_state.db
    .query_all(Statement::from_string(sea_orm::DatabaseBackend::Postgres, "Select * from user; ")).await.unwrap();

    api_response::ApiResponse::new(200, "Test".to_string())
}
