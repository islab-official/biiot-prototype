pub const HTTP_OPTIONS: &str =
    "HTTP/1.1 200 OK\r\n\
    date: Mon, 7 Jun 2021 23:33 GMT\r\n\
    Access-Control-Allow-Origin: *\r\n
    Access-Control-Allow-Headers: Content-Type\r\n\
    Allow: POST\r\n\
    Access-Control-Max-Age: 86400\r\n\
    Keep-Alive: timeout=2, max=99\r\n\
    content-length: 0\r\n\
    content-type: text/html\r\n\r\n";

pub const HTTP_HEADER_TEMPLATE: &str =
    "HTTP/1.1 200 OK\r\n\
    date: {}, {} {} {} {} GMT\r\n\
    content-type: application/json\r\n\r\n";

// pub fn http_options_response() -> &String {}