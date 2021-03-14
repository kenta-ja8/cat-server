use crate::foundation::server_object::{MessageContent, ResBody};
use actix_service::Service;
use actix_session::{Session, UserSession};
use actix_web::error;
use actix_web::http::{header, HeaderName, HeaderValue, StatusCode};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, HttpResponse};
use actix_web::{HttpRequest, Result};
use log;
use std::fs::File;
use std::{collections::HashMap, future::Future, io::Read};

pub fn call_api_counter_middleware(
    request: ServiceRequest,
    service: &mut impl Service<
        Request = ServiceRequest,
        Response = ServiceResponse,
        Error = actix_web::Error,
    >,
) -> impl Future<Output = Result<ServiceResponse, actix_web::Error>> {
    log::debug!("You requested: {}", request.path());
    let fut = service.call(request);
    async move {
        let mut res = fut.await?;
        let session: Session = res.request().get_session(); // https://github.com/actix/actix-web/issues/1790
        let key = "counter";
        let old_count = session.get::<i32>(key).unwrap_or(Some(0)).unwrap_or(0);
        let new_count = old_count + 1;
        session.set(key, new_count).unwrap();
        res.headers_mut().insert(
            HeaderName::from_static("x-api-counter"), // NG uppercase
            HeaderValue::from_str(&new_count.to_string()).unwrap(),
        );
        Ok(res)
    }
}

pub fn html_404_middleware(
    request: ServiceRequest,
    service: &mut impl Service<
        Request = ServiceRequest,
        Response = ServiceResponse,
        Error = actix_web::Error,
    >,
) -> impl Future<Output = Result<ServiceResponse, actix_web::Error>> {
    let fut = service.call(request);
    async move {
        let mut res = fut.await?;
        if res.status() != StatusCode::NOT_FOUND {
            return Ok(res);
        }
        log::error!("not exist url");
        res.headers_mut()
            .insert(header::CONTENT_TYPE, HeaderValue::from_static("text/html"));
        let resp = res.map_body(|_head, _body| {
            let mut fh = File::open("html/404.html").unwrap();
            let mut buf: Vec<u8> = vec![];
            let _ = fh.read_to_end(&mut buf);
            HttpResponse::build(StatusCode::NOT_FOUND)
                .body(buf)
                .take_body()
        });
        Ok(resp)
    }
}

pub fn validation_error_handler(
    err: actix_web_validator::Error,
    _request: &HttpRequest,
) -> actix_http::error::Error {
    let mut message_list = vec![];
    match &err {
        actix_web_validator::Error::Validate(validation_errors) => {
            for (field, validation_error_vec) in validation_errors.field_errors() {
                for (_num, validation_error) in validation_error_vec.iter().enumerate() {
                    message_list.push(MessageContent::new(
                        field,
                        validation_error.message.as_ref().unwrap().as_ref(),
                    ));
                    log::debug!(
                        "validator error: {} -> {}",
                        field,
                        validation_error.message.as_ref().unwrap()
                    );
                }
            }
        }
        actix_web_validator::Error::Deserialize(_error) => {
            message_list.push(MessageContent::new_only_message("deserialize error"));
        }
        actix_web_validator::Error::JsonPayloadError(_error) => {
            message_list.push(MessageContent::new_only_message("json payload error"));
        }
    }
    let content: HashMap<&str, &str> = HashMap::new();
    let res = ResBody {
        content: content,
        message_list: message_list,
    };
    error::InternalError::from_response(err, HttpResponse::BadRequest().json(res)).into()
}
