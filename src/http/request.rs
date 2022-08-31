use std::{
    error::Error,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    str::{self, Utf8Error},
};

use super::{
    method::{Method, MethodError},
    QueryString, QueryStringValue,
};

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidMethod)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidMethod)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidMethod)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let mut query_string = None;

        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            method: method.parse()?,
            path: path,
            query_string,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    None
}

// impl<'buf> Debug for Request<'buf> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
//         let mut query = String::from("");

//         if let Some(s) = &self.query_string {
//             query.push('{');

//             for ele in s.data.iter() {
//                 query.push(' ');
//                 query.push_str(ele.0);
//                 query.push(':');

//                 match ele.1 {
//                     QueryStringValue::Single(val) => {
//                         query.push(' ');
//                         query.push_str(val);
//                     }

//                     QueryStringValue::Multiple(vec) => {
//                         query.push_str(" [");

//                         for ele2 in vec.iter() {
//                             query.push(' ');
//                             query.push_str(ele2);
//                             query.push(',');
//                         }

//                         query.pop();
//                         query.push_str(" ]");
//                     }
//                 }

//                 query.push(',');
//             }

//             query.pop();
//             query.push_str(" }");
//         } else {
//             query.push_str("None");
//         }

//         write!(
//             f,
//             "Request {{\n\t Method: {} \n\t Path: {} \n\t Query string: {}\n}}",
//             &self.method, &self.path, query
//         )
//     }
// }

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}
