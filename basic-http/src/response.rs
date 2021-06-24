use crate::header::HttpHeader;
use crate::body::HttpBody;
use std::net::{TcpStream, Shutdown};
use std::io::Write;
use std::borrow::Borrow;
use crate::status::{HttpStatusCode, code_as_str_name};

pub struct HttpResponse {
    code: HttpStatusCode,
    header: HttpHeader,
    body: HttpBody,
    stream: TcpStream,      // TcpStream for writing response
}

impl ToString for HttpResponse {
    fn to_string(&self) -> String {
        let mut result = String::new();
        let status_str =
            format!("HTTP/1.1 {} {}\r\n", self.code as usize, code_as_str_name(&self.code));
        let now =
            chrono::DateTime::<chrono::Utc>::from_utc(chrono::Utc::now().naive_utc(), chrono::Utc);
        let now_str: String = now.to_rfc2822().replace("+0000", "GMT");
        let date = format!("date: {}\r\n", now_str);
        let content_length = format!("content-length: {}\r\n", self.body.data().len());
        result.push_str(status_str.as_str());
        result.push_str(date.as_str());
        result.push_str(content_length.as_str());
        result.push_str(self.header.to_string().as_str());
        if !self.get_header("content-type".to_lowercase().as_str()) {
            result = result.replace("\r\n\r\n", "\r\n");
            result.push_str("content-type: text/plain; charset=utf-8\r\n");
        }
        result.push_str("\r\n");
        result.push_str(self.body.to_string().as_str());
        result
    }
}

impl HttpResponse {
    // pub fn new(code: usize, header: &HttpHeader, body: &HttpBody) -> Self {
    //     return HttpResponse {
    //         code,
    //         header: header.clone(),
    //         body: body.clone(),
    //     }
    // }

    pub fn from_stream(stream: TcpStream) -> Self {
        HttpResponse {
            code: HttpStatusCode::Ok.clone(),
            header: HttpHeader::default(),
            body: HttpBody::default(),
            stream
        }
    }

    pub fn set_code(&mut self, code: HttpStatusCode) {
        self.code = code.clone();
    }

    pub fn get_header(&self, key: &str) -> bool {
        for kv in self.headers() {
            if kv.0.as_str() == key { return true; }
        }
        return false;
    }

    pub fn add_header(&mut self, key: &str, value: &str) {
        match key.to_ascii_lowercase().as_str() {
            "content-length" => {}
            "date" => {}
            _ => {
                self.header.add_header(key, value);
            }
        };
    }

    pub fn remove_header(&mut self, key: &str) -> bool {
        return match key.to_ascii_lowercase().as_str() {
            "content-length" => false,
            "date" => false,
            _ => self.header.remove_header(key)
        }
    }

    pub fn headers(&self) -> Vec<(String, String)> {
        self.header.headers()
    }

    pub fn write_str(&mut self, data: String) {
        self.body.append_str(data);
        // self.stream.write(data.as_ref());
    }

    pub fn write_json(&mut self, data: serde_json::Value) {
        self.body.append_json(data);
    }

    pub fn set_data(&mut self, data: &str) {
        self.body.set_data(data);
    }

    pub fn clear_buffer(&mut self) {
        self.body.clear();
    }

    pub fn send(&mut self) {
        let mut result = self.to_string();
        self.stream.write(result.as_bytes());
        self.stream.shutdown(Shutdown::Both);
    }
}