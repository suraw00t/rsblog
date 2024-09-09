use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Bad Request: {0}")]
    BadRequest(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Payment Required: {0}")]
    PaymentRequired(String),
    #[error("Forbidden: {0}")]
    Forbidden(String),
    #[error("Not Found: {0}")]
    NotFound(String),
    #[error("Method Not Allowed: {0}")]
    MethodNotAllowed(String),
    #[error("Not Acceptable: {0}")]
    NotAcceptable(String),
    #[error("Proxy Authentication Required: {0}")]
    ProxyAuthenticationRequired(String),
    #[error("Request Timeout: {0}")]
    RequestTimeout(String),
    #[error("Conflict: {0}")]
    Conflict(String),
    #[error("Gone: {0}")]
    Gone(String),
    #[error("Length Required: {0}")]
    LengthRequired(String),
    #[error("Precondition Failed: {0}")]
    PreconditionFailed(String),
    #[error("Payload Too Large: {0}")]
    PayloadTooLarge(String),
    #[error("URI Too Long: {0}")]
    UriTooLong(String),
    #[error("Unsupported Media Type: {0}")]
    UnsupportedMediaType(String),
    #[error("Range Not Satisfiable: {0}")]
    RangeNotSatisfiable(String),
    #[error("Expectation Failed: {0}")]
    ExpectationFailed(String),
    #[error("I'm a teapot: {0}")]
    ImATeapot(String),
    #[error("Misdirected Request: {0}")]
    MisdirectedRequest(String),
    #[error("Unprocessable Entity: {0}")]
    UnprocessableEntity(String),
    #[error("Locked: {0}")]
    Locked(String),
    #[error("Failed Dependency: {0}")]
    FailedDependency(String),
    #[error("Upgrade Required: {0}")]
    UpgradeRequired(String),
    #[error("Precondition Required: {0}")]
    PreconditionRequired(String),
    #[error("Too Many Requests: {0}")]
    TooManyRequests(String),
    #[error("Request Header Fields Too Large: {0}")]
    RequestHeaderFieldsTooLarge(String),
    #[error("Unavailable For Legal Reasons: {0}")]
    UnavailableForLegalReasons(String),
    #[error("Internal Server Error: {0}")]
    InternalServerError(String),
    #[error("Not Implemented: {0}")]
    NotImplemented(String),
    #[error("Bad Gateway: {0}")]
    BadGateway(String),
    #[error("Service Unavailable: {0}")]
    ServiceUnavailable(String),
    #[error("Gateway Timeout: {0}")]
    GatewayTimeout(String),
    #[error("HTTP Version Not Supported: {0}")]
    HttpVersionNotSupported(String),
    #[error("Variant Also Negotiates: {0}")]
    VariantAlsoNegotiates(String),
    #[error("Insufficient Storage: {0}")]
    InsufficientStorage(String),
    #[error("Loop Detected: {0}")]
    LoopDetected(String),
    #[error("Not Extended: {0}")]
    NotExtended(String),
    #[error("Network Authentication Required: {0}")]
    NetworkAuthenticationRequired(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let error_message = self.to_string();
        let error_response = ErrorResponse {
            error: error_message,
        };

        HttpResponse::build(self.status_code()).json(error_response)
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ApiError::PaymentRequired(_) => StatusCode::PAYMENT_REQUIRED,
            ApiError::Forbidden(_) => StatusCode::FORBIDDEN,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::MethodNotAllowed(_) => StatusCode::METHOD_NOT_ALLOWED,
            ApiError::NotAcceptable(_) => StatusCode::NOT_ACCEPTABLE,
            ApiError::ProxyAuthenticationRequired(_) => StatusCode::PROXY_AUTHENTICATION_REQUIRED,
            ApiError::RequestTimeout(_) => StatusCode::REQUEST_TIMEOUT,
            ApiError::Conflict(_) => StatusCode::CONFLICT,
            ApiError::Gone(_) => StatusCode::GONE,
            ApiError::LengthRequired(_) => StatusCode::LENGTH_REQUIRED,
            ApiError::PreconditionFailed(_) => StatusCode::PRECONDITION_FAILED,
            ApiError::PayloadTooLarge(_) => StatusCode::PAYLOAD_TOO_LARGE,
            ApiError::UriTooLong(_) => StatusCode::URI_TOO_LONG,
            ApiError::UnsupportedMediaType(_) => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            ApiError::RangeNotSatisfiable(_) => StatusCode::RANGE_NOT_SATISFIABLE,
            ApiError::ExpectationFailed(_) => StatusCode::EXPECTATION_FAILED,
            ApiError::ImATeapot(_) => StatusCode::IM_A_TEAPOT,
            ApiError::MisdirectedRequest(_) => StatusCode::MISDIRECTED_REQUEST,
            ApiError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            ApiError::Locked(_) => StatusCode::LOCKED,
            ApiError::FailedDependency(_) => StatusCode::FAILED_DEPENDENCY,
            ApiError::UpgradeRequired(_) => StatusCode::UPGRADE_REQUIRED,
            ApiError::PreconditionRequired(_) => StatusCode::PRECONDITION_REQUIRED,
            ApiError::TooManyRequests(_) => StatusCode::TOO_MANY_REQUESTS,
            ApiError::RequestHeaderFieldsTooLarge(_) => StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE,
            ApiError::UnavailableForLegalReasons(_) => StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
            ApiError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::NotImplemented(_) => StatusCode::NOT_IMPLEMENTED,
            ApiError::BadGateway(_) => StatusCode::BAD_GATEWAY,
            ApiError::ServiceUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
            ApiError::GatewayTimeout(_) => StatusCode::GATEWAY_TIMEOUT,
            ApiError::HttpVersionNotSupported(_) => StatusCode::HTTP_VERSION_NOT_SUPPORTED,
            ApiError::VariantAlsoNegotiates(_) => StatusCode::VARIANT_ALSO_NEGOTIATES,
            ApiError::InsufficientStorage(_) => StatusCode::INSUFFICIENT_STORAGE,
            ApiError::LoopDetected(_) => StatusCode::LOOP_DETECTED,
            ApiError::NotExtended(_) => StatusCode::NOT_EXTENDED,
            ApiError::NetworkAuthenticationRequired(_) => {
                StatusCode::NETWORK_AUTHENTICATION_REQUIRED
            }
        }
    }
}
