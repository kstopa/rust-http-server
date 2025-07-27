use super::QueryString;
use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::Utf8Error;

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    // return the next word and the rest of the string or Nothing

    // Example of loop with an iterator -> but better use for loops
    // request.chars();
    // loop {
    //     let item = iter.next();
    //     match item {
    //         Some(c) => {}
    //         None => break,
    //     }
    // }

    for (i, c) in request.chars().enumerate() {
        // enumerate gets the index and val
        if c == ' ' || 'c' == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    None
}

/// Represents an HTTP request.
/// GET /index.html HTTP/1.1
/// Host: www.example.re
/// User-Agent: Mozilla/5.0 (Windows; U; Windows NT 5.0; en-US; rv:1.1)
/// Accept: text/html
/// Accept-Language: en-US, en; q=0.5
/// Accept-Encoding: gzip, deflate

#[derive(Debug)] // Implements Debug trait
pub struct Request<'buf> {
    path: &'buf str,
    query: Option<QueryString<'buf>>, // Options are values or None
    method: Method,
}

// Getters
impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query(&self) -> Option<&QueryString> {
        self.query.as_ref()
    }
}

/// Parsses an HTTP request from a byte slice.
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {
        // match str::from_utf8(buf) {
        //    Ok(request) => {}
        //    Err(_) => return Err(ParseError::InvalidEncoding),
        // }

        // Same as VVVVVV

        // match str::from_utf8(buf).or(Err(ParseError::InvalidMethod)) {
        //     Ok(request) => {
        //         unimplemented!()
        //     }
        //     Err(e) => return Err(e),
        // }

        // That is the same as VVVVV
        // let request = str::from_utf8(buf).or(Err(ParseError::InvalidMethod))?;

        // That is the same as VVVVV
        let request = str::from_utf8(buf)?;

        // match get_next_word(request) {
        //     Some((method, request)) => {}
        //     None => return Err(ParseError::InvalidRequest),
        // }
        // That is same as VVVVV
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, request) = get_next_word(request).ok_or(ParseError::InvalidProtocol)?;

        // Rest of the request is ignored (headers, body)
        if protocol == "HTTP/1.1" {
            println!("PRotocol {}", protocol);
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query = None;
        // match path.find('?') {
        //     Some(index) => {
        //         query_string = Some(&path[index + 1..].to_string());
        //         path = &path[..index];
        //     }
        //     None => {}
        //}

        // Same as VVVVV
        if let Some(i) = path.find('?') {
            query = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            path,
            query,
            method,
        })
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

// To enable automatic conversion from the question mark instead of ussing or.
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
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}
