use serde::Deserialize;
use url::Url;

use crate::engines::{EngineResponse, EngineSearchResult, RequestResponse, CLIENT};

pub fn request(query: &str) -> RequestResponse {
    CLIENT
        .get(
            Url::parse_with_params(
                "https://fdn.qwant.com/v3/search/web",
                &[
                    ("q", query),
                    ("freshness", "any"),
                    ("count", "10"),
                    ("locale", "en_US"),
                    ("offset", "0"),
                    ("device", "desktop"),
                    ("tgp", "3"),
                    ("safesearch", "0"),
                    ("displayed", "true"),
                ],
            )
            .unwrap()
            .as_str(),
        )
        .into()
}

#[derive(Deserialize, Debug)]
struct QwantSearchItem {
    pub title: String,
    pub url: String,
    pub desc: String,
}

pub fn parse_response(body: &str) -> eyre::Result<EngineResponse> {
    let mut results: Vec<QwantSearchItem> = Vec::new();
    match serde_json::from_str::<serde_json::Value>(body) {
        Ok(json_val) => {
            if let Some(data) = json_val.get("data") {
                if let Some(result) = data.get("result") {
                    if let Some(items) = result.get("items") {
                        if let Some(mainline) = items.get("mainline").and_then(|o| o.as_array()) {
                            for group in mainline {
                                if let Some(t) = group.get("type").and_then(|o| o.as_str()) {
                                    if t == "web" {
                                        if let Some(items) =
                                            group.get("items").and_then(|o| o.as_array())
                                        {
                                            for i in items {
                                                let item: QwantSearchItem =
                                                    serde_json::from_value(i.clone()).unwrap();
                                                results.push(item);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(_) => {
            return Ok(EngineResponse::new());
        }
    }

    let search_results = results
        .into_iter()
        .map(|result| EngineSearchResult {
            url: result.url,
            title: result.title,
            description: result.desc,
        })
        .collect();

    Ok(EngineResponse {
        search_results,
        featured_snippet: None,
        answer_html: None,
        infobox_html: None,
    })
}
