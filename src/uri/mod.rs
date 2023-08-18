use actix_web::HttpRequest;
use url::{ParseError, Url};

pub(crate) struct RequestParser<'a> {
    request: &'a HttpRequest,
    pub path: String
}

impl RequestParser<'_> {
    pub fn new(request: &HttpRequest) -> RequestParser {
        RequestParser {
            request,
            path: request.path().to_string()
        }
    }
    pub fn parse(&self) -> Result<Url, ParseError> {
        let mut uri = self.request.connection_info().scheme().to_string();
        uri.push_str("://");
        uri.push_str(self.request.connection_info().host());
        let url = Url::parse(uri.as_str());
        url
    }
    pub(crate) fn ends_with_slash(&self) -> bool {
        self.path.ends_with("/")
    }

    pub(crate) fn tail(&self) -> String{
        self.request.match_info().get("tail").unwrap().parse().unwrap()
    }
    pub fn get_nodes(&self) -> Vec<String> {
        let tail = self.tail();
        let nodes: Vec<String> = tail
            .split("/")
            .map(|s| s.to_string())
            .filter(|v| !v.is_empty())
            .collect();
        nodes
    }
}