use actix_web::HttpRequest;
use url::{ParseError, Url};

pub(crate) struct UriParser;

impl UriParser {
    pub fn parse(request: HttpRequest) -> Result<Url, ParseError> {
        let mut uri = request.connection_info().scheme().to_string();
        uri.push_str("://");
        uri.push_str(request.connection_info().host());
        let url = Url::parse(uri.as_str());
        url
    }
}