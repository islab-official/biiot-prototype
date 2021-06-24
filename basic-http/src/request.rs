use std::collections::HashMap;
use crate::header::HttpHeader;
use crate::body::HttpBody;
use crate::status::HttpStatus;
use std::borrow::Borrow;
use regex::Regex;

pub struct RawRequest {
    buffer: [u8;8096],
    length: usize,
}

impl RawRequest {
    pub fn new(buffer: [u8;8096], length: usize) -> Self {
        RawRequest { buffer, length }
    }
}

pub struct HttpRequest {
    status: HttpStatus,
    header: HttpHeader,
    body: HttpBody,
}

impl HttpRequest {
    pub fn from_raw_request(request: RawRequest) -> Result<Self, ()> {
        let raw_request =
            String::from_utf8_lossy(&request.buffer[..request.length]);
        let mut request_split = raw_request.split("\r\n\r\n");
        let maybe_header = request_split.next();
        let maybe_body = request_split.next();
        let mut request_body = "";
        if maybe_body.is_some() {
            // some broken message does not contain body
            request_body = maybe_body.unwrap();
        }

        let result_header_and_status =
            extract_header(maybe_header.unwrap());
        return match result_header_and_status {
            Ok(header_and_status) => {
                let body =
                    extract_body(request_body);

                Ok(HttpRequest {
                    status: header_and_status.0,
                    header: header_and_status.1,
                    body
                })
            }
            Err(_) => {
                Err(())
            }
        }

    }
}

impl ToString for HttpRequest {
    fn to_string(&self) -> String {
        let mut result = String::new();
        let status_str =
            format!("{} {} {}\r\n", self.status.method(), self.path(), self.version());
        result.push_str(status_str.as_str());
        result.push_str(self.header.to_string().as_str());
        result.push_str(self.body.to_string().as_str());
        result
    }
}

impl HttpRequest {

    pub fn method(&self) -> &str { self.status.method() }

    pub fn path(&self) -> &str { self.status.path() }

    pub fn header(&self) -> &HttpHeader { &self.header }

    pub fn version(&self) -> &str { &self.status.version() }

    pub fn body(&self) -> &HttpBody { &self.body }
}

fn extract_start_line(start_line_literal: &str) -> Result<HttpStatus, ()> {

    if start_line_literal.trim().len() == 0 {
        // receive empty string.
        return Err(());
    }
    let re =
        Regex::new(r"((GET|POST|PUT|PATCH|DELETE|OPTIONS|HEAD|CONNECT|TRACE)\s\S+\sHTTP/\d\.\d)")
            .unwrap();
    if !re.is_match(start_line_literal) {
        println!("invalid HTTP request detected.. {}", start_line_literal);
        return Err(());
    }

    let mut tokens = start_line_literal.split(" ");
    let method = tokens.next().unwrap();
    let path = tokens.next().unwrap();
    let version = tokens.next().unwrap();
    return Ok(HttpStatus::new(200, method, path, version));
}

fn extract_header(header_literal: &str) -> Result<(HttpStatus, HttpHeader), ()> {
    let mut map = HashMap::<String, String>::new();
    let mut lines = header_literal.split("\r\n");
    let status = extract_start_line(lines.next().unwrap());

    if status.is_err() { return Err(()); }

    for line in lines.into_iter() {
        let mut tokens = line.split(": ");
        map.insert(
            tokens.next().unwrap().to_string(),
            tokens.next().unwrap().to_string()
        );
    }
    return Ok(
        (status.unwrap(), HttpHeader::from(map))
    );
}

fn extract_body(body_literal: &str) -> HttpBody {
    HttpBody::from(body_literal)
}