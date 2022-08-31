use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    OPTIONS,
    HEAD,
    CONNECT,
    TRACE,
    PATCH,
}

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "DELETE" => Ok(Self::DELETE),
            "OPTIONS" => Ok(Self::OPTIONS),
            "HEAD" => Ok(Self::HEAD),
            "CONNECT" => Ok(Self::CONNECT),
            "TRACE" => Ok(Self::TRACE),
            "PATCH" => Ok(Self::PATCH),
            _ => Err(MethodError),
        }
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Self::GET => "GET".to_string(),
            Self::POST => "POST".to_string(),
            Self::PUT => "PUT".to_string(),
            Self::DELETE => "DELETE".to_string(),
            Self::OPTIONS => "OPTIONS".to_string(),
            Self::HEAD => "HEAD".to_string(),
            Self::CONNECT => "CONNECT".to_string(),
            Self::TRACE => "TRACE".to_string(),
            Self::PATCH => "PATCH".to_string(),
        };

        write!(f, "{}", string)
    }
}

pub struct MethodError;
