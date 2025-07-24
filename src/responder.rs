use actix_web::{HttpResponse, Responder, ResponseError, body::BoxBody};
use serde::Serialize;

use crate::{ApiErr, macros::ApiRes};

#[derive(Serialize)]
struct ApiJsonBody<'a, T: Serialize> {
    /// Status code
    code: u32,
    /// Message describing the response
    message: &'a str,
    /// Optional data included in the response
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<&'a T>,
}

impl<T: Serialize> Responder for ApiRes<T> {
    type Body = BoxBody;
    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse {
        let status = self.status();
        let body = ApiJsonBody {
            code: self.code(),
            message: self.message(),
            data: self.data(),
        };
        HttpResponse::build(status).json(body)
    }
}

#[derive(Serialize)]
struct ApiJsonError<'a> {
    /// Status code
    code: u32,
    /// Error message
    message: &'a str,
}

impl ResponseError for ApiErr {
    fn error_response(&self) -> HttpResponse {
        let body = ApiJsonError {
            code: self.code(),
            message: self.message(),
        };
        HttpResponse::build(self.status()).json(body)
    }
}
