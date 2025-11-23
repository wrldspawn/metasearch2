use url::Url;

use crate::{
    engines::{EngineResponse, RequestResponse, CLIENT},
    parse::{parse_html_response_with_opts, ParseOpts},
};

pub async fn request(query: &str) -> RequestResponse {
    CLIENT
        .get(
            Url::parse_with_params(
                "https://www.ecosia.org/search",
                &[("method", "index"), ("q", query)],
            )
            .unwrap()
            .as_str(),
        )
        .into()
}

pub fn parse_response(body: &str) -> eyre::Result<EngineResponse> {
    parse_html_response_with_opts(
        body,
        ParseOpts::new()
            .result(".result__body")
            .title(".result-title__heading")
            .href(".result__link")
            .description(".web-result__description"),
    )
}
