use actix_web::{body::BoxBody, http::StatusCode, web, HttpResponse, Responder};



pub struct ApiResponse{
    pub status_code: u16,
    pub body: String,
    response_code: StatusCode
}

impl ApiResponse{
    pub fn new(status_code: u16, body: String) -> Self {
        ApiResponse{
            status_code,
            body,
            response_code: StatusCode::from_u16(status_code).unwrap()
        }
    }
}

impl Responder for ApiResponse{
    type Body = BoxBody;

    fn respond_to(self, req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = BoxBody::new(web::BytesMut::from(self.body.as_bytes()));
        HttpResponse::new(self.response_code).set_body(body)
    }
}