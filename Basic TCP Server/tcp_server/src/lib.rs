pub enum HttpResponse {
    Ok,
    NotFound,
}

impl HttpResponse {
    pub fn to_string(&self) -> &str{
        match self {
            Self::Ok => "HTTP/1.1 200 OK",
            Self::NotFound => "HTTP/1.1 404 NOT FOUND"
        }
    }

    pub fn return_content(status_line: &str,content_len: usize, content: String ) -> String{
        format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, content_len, content).to_string()
    }
}