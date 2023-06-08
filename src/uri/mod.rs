use actix_web::HttpRequest;
use url::{ParseError, Url};

pub(crate) struct UriParser<'a> {
    request: &'a HttpRequest,
    pub path: String
}

impl UriParser<'_> {
    pub fn new(request: &HttpRequest) -> UriParser {
        UriParser {
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
    pub fn get_nodes(&self) -> Vec<String> {
        let nodes: Vec<String> = self.path
            .split("/")
            .map(|s| s.to_string())
            .filter(|v| !v.is_empty())
            .collect();
        nodes
    }
    pub(crate) fn ends_with_slash(&self) -> bool {
        self.path.ends_with("/")
    }
}