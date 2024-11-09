use std::collections::{btree_map::Keys, HashMap};

use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, BufReader},
    net::TcpStream,
    stream,
};

use super::error::Error;

#[derive(Debug)]
pub enum Method {
    Get,
    Post,
    Options,
    Unknow,
}

impl From<&str> for Method {
    fn from(value: &str) -> Self {
        match value {
            "OPTIONS" => Method::Options,
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Unknow,
        }
    }
}
#[derive(Debug)]
pub enum Proto {
    HTTP09,
    HTTP10,
    HTTP11,
    H2,
    H3,
    Unknow,
}

impl From<&str> for Proto {
    fn from(value: &str) -> Self {
        match value {
            "HTTP/0.9" => Proto::HTTP09,
            "HTTP/1.0" => Proto::HTTP10,
            "HTTP/1.1" => Proto::HTTP11,
            "HTTP/2.0" => Proto::H2,
            "HTTP/3.0" => Proto::H3,
            _ => Proto::Unknow,
        }
    }
}
#[derive(Debug)]
pub struct Url {
    pub path: String,
    pub query: Option<String>, //可以没有
}

impl Url {
    pub fn from(full: &str) -> Result<Url,Error> {
        let mut split_url = full.splitn(2, '?');
        let path = split_url.next().ok_or(Error::new("url not value"))?.to_string();
        let query = split_url.next().map(|s| s.to_string());

        Ok(Url { path, query })
    }
    pub fn query(&self) -> Option<Value> {
        let q = self.query.as_ref()?;
        Some(Value::parse(q))
    }
}

pub struct Value<'a> {
    q: &'a str,
}
impl<'a> Value<'a> {
    pub fn parse(q: &str) -> Value {
        Value { q }
    }
}
impl<'a> Iterator for Value<'a> {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.q.is_empty() {
                return None;
            }
            // a=b & c=d&e=f
            let mut kv = self.q.splitn(2, '&');
            let sequence = kv.next().unwrap();
            if sequence.is_empty() {
                continue;
            }
            self.q = kv.next().unwrap_or("");
            let mut kv = sequence.splitn(2, '=');
            let name = kv.next().unwrap().to_string();
            let value = kv.next().unwrap_or("").to_string();

            return Some((name, value));
        }
    }
}

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub url: Url,
    pub proto: Proto,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

impl Request {
    //url部分
    pub async fn from_stream(stream: &mut TcpStream) -> Result<Request, Error> {
        let mut buf_reader = BufReader::new(stream).lines();
        //请求行

        let request_line = buf_reader
            .next_line()
            .await
            .map_err(|err| Error::new(err))?
            .ok_or(Error::new("request line is empty"))?;

        let mut words = request_line.split_whitespace();

        let method = words.next().ok_or(Error::new("method is empty"))?.into();

        let full = words.next().ok_or(Error::new("url is empty"))?;

        let url = Url::from(full)?;

        let proto = words.next().ok_or(Error::new("proto is empty"))?.into();
        //http版本部分

        //请求头
        let mut headers = HashMap::new();
        loop {
            let header = buf_reader
                .next_line()
                .await
                .map_err(|err| Error::new(err))?
                .ok_or(Error::new("header is empty"))?;

            if header.is_empty() {
                break;
            }
            let mut split_hl = header.splitn(2, ":");
            if let (Some(key), Some(value)) = (split_hl.next(), split_hl.next()) {
                headers.insert(
                    key.trim().to_lowercase().to_string(),
                    value.trim().to_lowercase().to_string(),
                );
            }
        }

        //请求体，可以拥有换行
        let body = if let Some(content_length) = headers.get("content-length") {
            let mut body = vec![0; content_length.parse().unwrap()];
            buf_reader.get_mut().read_exact(&mut body).await.unwrap();
            Some(body)
        } else {
            None
        };

        Ok(Request {
            method,
            url,
            proto,
            headers,
            body,
        })
    }
}
