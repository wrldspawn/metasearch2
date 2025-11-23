use serde::Deserialize;
use url::Url;

use crate::engines::{EngineResponse, EngineSearchResult, RequestResponse, CLIENT};

pub async fn request(query: &str) -> RequestResponse {
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
            if let Some(mainline) = json_val
                .pointer("/data/result/items/mainline")
                .and_then(|o| o.as_array())
            {
                results.extend(
                    mainline
                        .iter()
                        .filter(|o| o["type"] == "web")
                        .flat_map(|o| o["items"].as_array().into_iter().flatten())
                        .map(|item| serde_json::from_value(item.clone()).unwrap()),
                );
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
