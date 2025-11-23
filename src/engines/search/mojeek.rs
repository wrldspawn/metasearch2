use url::Url;

use crate::{
    engines::{EngineResponse, RequestResponse, CLIENT},
    parse::{parse_html_response_with_opts, ParseOpts},
};

pub async fn request(query: &str) -> RequestResponse {
    CLIENT
        .get(
            Url::parse_with_params("https://www.mojeek.com/search", &[("q", query)])
                .unwrap()
                .as_str(),
        )
        .into()
}

pub fn parse_response(body: &str) -> eyre::Result<EngineResponse> {
    parse_html_response_with_opts(
        body,
        ParseOpts::new()
            .result(".results-standard > li")
            .title("h2 > a")
            .href("h2 > a")
            .description("p.s"),
    )
}
