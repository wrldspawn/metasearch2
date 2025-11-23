use scraper::{ElementRef, Selector};
use url::Url;

use crate::{
    engines::{EngineResponse, RequestResponse, CLIENT},
    parse::{parse_html_response_with_opts, ParseOpts, QueryMethod},
};

pub async fn request(query: &str) -> RequestResponse {
    CLIENT
        .get(
            Url::parse_with_params(
                "https://html.duckduckgo.com/html/",
                &[("q", query), ("kl", "wt-wt"), ("kp", "-1")],
            )
            .unwrap()
            .as_str(),
        )
        .header("Host", "html.duckduckgo.com")
        .header("Alt-Used", "html.duckduckgo.com")
        .into()
}

pub fn parse_response(body: &str) -> eyre::Result<EngineResponse> {
    parse_html_response_with_opts(
        body,
        ParseOpts::new()
            .result(".results > .web-result")
            .title(".result__title")
            .href(QueryMethod::Manual(Box::new(|el: &ElementRef| {
                let url = el
                    .select(&Selector::parse("a.result__a").unwrap())
                    .next()
                    .and_then(|n| n.value().attr("href"))
                    .unwrap_or_default();
                clean_url(url)
            })))
            .description(".result__snippet"),
    )
}

fn clean_url(url: &str) -> eyre::Result<String> {
    if url.starts_with("//duckduckgo.com/l/?uddg=") {
        let url = Url::parse(&format!("https:{url}"))?;
        let u = url
            .query_pairs()
            .find(|(key, _)| key == "uddg")
            .unwrap_or_default()
            .1;
        Ok(String::from(u))
    } else {
        Ok(url.to_string())
    }
}
