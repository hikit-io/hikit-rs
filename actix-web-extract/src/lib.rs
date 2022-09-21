use actix_web::{
    error::ParseError,
    http::{header, header::Header},
};
use std::{
    fmt,
    future::{ready, Ready},
};

#[derive(Debug)]
pub struct FirstAcceptLanguage(header::AcceptLanguage);

impl fmt::Display for FirstAcceptLanguage {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        for lang in self.0.as_slice() {
            return write!(f, "{}", lang.to_string());
        }
        write!(f, "{}", "")
    }
}

impl actix_web::FromRequest for FirstAcceptLanguage {
    type Error = ParseError;
    type Future = Ready<Result<Self, Self::Error>>;

    #[inline]
    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        match Header::parse(req) {
            Ok(header) => ready(Ok(FirstAcceptLanguage(header))),
            Err(e) => ready(Err(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
