use super::method::{MethodError, Method};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Result as FmtResult;
use std::fmt::Formatter;
use std::str;
use std::str::Utf8Error;

pub struct Request<'a> {

    path: &'a str,
    query_string: Option<&'a str>,
    method: Method,

}

// impl Request {
//     fn from_byte_array(buf: &[u8]) -> Result<Self, String> {}
// }

impl<'a> TryFrom<&'a [u8]> for Request<'a> {
    type Error = ParseError;

    fn try_from(buf: &'a [u8]) -> Result<Self, Self::Error> {

        // match str::from_utf8(buf) {
        //     Ok(request) => {},
        //     Err(_) => return Err(ParseError::InvalidEncoding)
        // }

        let request = str::from_utf8(buf)?;

        let (method, request) = next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;

        // match path.find('?') {
        //     Some(i) => {
        //         query_string = Some(&path[i+1..]);
        //         path = &path[..i]
        //     }
        //     None => {}
        // }

        if let Some(i) = path.find('?') {
            query_string = Some(&path[i+1..]);
            path =&path[..i];
        }
        
        Ok(Self {
            path,
            query_string,
            method,
        })  

    }
}

fn next_word(request: &str) -> Option<(&str, &str)> {
    
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r'{
            return Some((&request[..i], &request[i+1..]));
        }
    }

    None

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
            Self::InvalidEncoding => "invalid encoding",
            Self::InvalidRequest => "invalid request",
            Self::InvalidProtocol => "invalid protocol",
            Self::InvalidMethod => "invalid method",
        }
    }
}
 
impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
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


impl Error for ParseError {


    
}
