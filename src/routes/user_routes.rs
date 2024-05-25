use actix_web::{middleware, web};
use actix_web_lab::middleware::from_fn;

use super::{handlers, middlewares};



pub fn config(config: &mut web::ServiceConfig){
    config
    .service(
        web::scope("/user")
        .wrap(from_fn(middlewares::auth_middleware::check_auth_middleware))
        .service(handlers::user_handlers::user)
        .service(handlers::user_handlers::update_user)
    );
}