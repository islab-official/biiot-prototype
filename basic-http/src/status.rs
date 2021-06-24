use std::collections::HashMap;

pub struct HttpStatusName {}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum HttpStatusCode {
    Ok              = 200,
    Created         = 201,
    Accepted        = 202,
    NoContent       = 204,
    BadRequest      = 400,
    Unauthorized    = 401,
    PaymentRequired = 402,
    Forbidden       = 403,
    NotFound        = 404,
    ServerError     = 500,
    NotImplemented  = 501,
    BadGateway      = 502,
}

pub fn code_as_str_name(code: &HttpStatusCode) -> &str {
    return match code {
        HttpStatusCode::Ok => "Ok",
        HttpStatusCode::Created => "Created",
        HttpStatusCode::Accepted => "Accepted",
        HttpStatusCode::NoContent => "No Content",
        HttpStatusCode::BadRequest => "Bad Request",
        HttpStatusCode::Unauthorized => "Unauthorized",
        HttpStatusCode::PaymentRequired => "Payment Required",
        HttpStatusCode::Forbidden => "Forbidden",
        HttpStatusCode::NotFound => "Not Found",
        HttpStatusCode::ServerError => "Internal Server Error",
        HttpStatusCode::NotImplemented => "Not Implemented",
        HttpStatusCode::BadGateway => "Bad Gateway"
    }
}

pub struct HttpStatus {
    code: usize,
    method: String,
    path: String,
    version: String,
}

impl Clone for HttpStatus {
    fn clone(&self) -> Self {
        HttpStatus {
            code: self.code.clone(),
            method: self.method.clone(),
            path: self.path.clone(),
            version: self.version.clone(),
        }
    }
}

// impl ToString for HttpStatus {
//     fn to_string(&self) -> String {
//         format!("{} {} {}\r\n", self.method, self.path, self.version)
//     }
// }

impl HttpStatus {
    pub fn new(code: usize, method: &str, path: &str, version: &str) -> Self {
        HttpStatus {
            code,
            method: method.to_string(),
            path: path.to_string(),
            version: version.to_string()
        }
    }

    pub fn code(&self) -> usize { self.code }

    pub fn method(&self) -> &str { self.method.as_str() }

    pub fn path(&self) -> &str { self.path.as_str() }

    pub fn version(&self) -> &str { self.version.as_str() }
}