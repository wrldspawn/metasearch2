use url::Url;

use crate::{
    engines::{EngineResponse, RequestResponse, CLIENT},
    parse::{parse_html_response_with_opts, ParseOpts},
};

pub fn request(query: &str) -> RequestResponse {
    CLIENT
        .get(
            Url::parse_with_params("https://www.dogpile.com/serp", &[("q", query)])
                .unwrap()
                .as_str(),
        )
        .into()
}

pub fn parse_response(body: &str) -> eyre::Result<EngineResponse> {
    parse_html_response_with_opts(
        body,
        ParseOpts::new()
            .result(".web-google__result")
            .title(".web-google__title")
            .href(".web-google__title")
            .description(".web-google__description"),
    )
}
