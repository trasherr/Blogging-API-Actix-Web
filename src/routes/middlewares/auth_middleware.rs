use actix_web::{body::MessageBody, dev::{ServiceRequest, ServiceResponse}, http::header::AUTHORIZATION, Error, HttpMessage};
use actix_web_lab::middleware::Next;

use crate::utils::{api_response::{self, ApiResponse}, jwt::decode_jwt};



pub async fn check_auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>
) -> Result<ServiceResponse<impl MessageBody>,Error> {
    let auth = req.headers().get(AUTHORIZATION);

    if auth.is_none() {
        return Err(Error::from(api_response::ApiResponse::new(401, "Unauthorised".to_string())));
    }

    let token = auth.unwrap().to_str().unwrap().replace("Bearer ", "").to_owned();
    let claim = decode_jwt(token).unwrap();
    req.extensions_mut().insert(claim.claims);
    
    next.call(req).await
    .map_err(|err| Error::from(ApiResponse::new(500, err.to_string())))
}