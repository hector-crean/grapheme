use crate::reference::reference_style::Reference;
use axum::Json;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct SearchQuery {
    q: String,
}

const PUBMED_BASE_URL: &str = "https://eutils.ncbi.nlm.nih.gov/entrez/eutils";

#[derive(Debug, Deserialize)]
struct PubMedSearchResult {
    #[serde(rename = "esearchresult")]
    result: ESearchResult,
}

#[derive(Debug, Deserialize)]
struct ESearchResult {
    #[serde(rename = "idlist")]
    id_list: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct PubMedSummaryResult {
    result: SummaryResult,
}

#[derive(Debug, Deserialize)]
struct SummaryResult {
    #[serde(rename = "uids")]
    uids: Vec<String>,
    #[serde(flatten)]
    articles: std::collections::HashMap<String, Article>,
}

#[derive(Debug, Deserialize)]
struct Article {
    uid: String,
    pubdate: String,
    #[serde(default)]
    epubdate: String,
    source: String,
    authors: Vec<Author>,
    lastauthor: String,
    title: String,
    #[serde(rename = "sortpubdate")]
    sort_pubdate: String,
    volume: String,
    issue: String,
    pages: String,
    lang: Vec<String>,
    #[serde(rename = "issn", default)]
    issn: String,
    #[serde(rename = "essn")]
    essn: String,
    pubtype: Vec<String>,
    #[serde(rename = "articleids")]
    article_ids: Vec<ArticleId>,
    // Add other fields as needed
}

#[derive(Debug, Deserialize)]
struct Author {
    name: String,
    authtype: String,
    clusterid: String,
}

#[derive(Debug, Deserialize)]
struct ArticleId {
    idtype: String,
    idtypen: i32,
    value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchResponse {
    pub references: Vec<Reference>,
}

pub async fn search_pubmed(query: &str) -> Result<SearchResponse, Box<dyn std::error::Error>> {
    let client = Client::new();

    // First, search for IDs
    let search_url = format!(
        "{}/esearch.fcgi?db=pubmed&term={}&retmode=json",
        PUBMED_BASE_URL, query
    );
    let search_response: PubMedSearchResult = client.get(&search_url).send().await?.json().await?;

    let ids = search_response.result.id_list.join(",");

    // Then, fetch summaries for these IDs
    let summary_url = format!(
        "{}/esummary.fcgi?db=pubmed&id={}&retmode=json",
        PUBMED_BASE_URL, ids
    );

    let rep = client.get(&summary_url).send().await?;
    println!("{:?}", rep);
    let summary_response: PubMedSummaryResult = rep.json().await?;

    let results: Vec<Reference> = summary_response
        .result
        .articles
        .values()
        .map(|article| Reference {
            authors: article.authors.iter().map(|a| a.name.clone()).collect(),
            year: article
                .pubdate
                .split_whitespace()
                .next()
                .and_then(|year| year.parse().ok()),
            title: article.title.clone(),
            container: Some(article.source.clone()),
            volume: article.volume.parse().ok(),
            issue: article.issue.parse().ok(),
            pages: Some(article.pages.clone()),
            publication_date: Some(article.pubdate.clone()),
            doi: article
                .article_ids
                .iter()
                .find(|id| id.idtype == "doi")
                .map(|id| id.value.clone()),
            // Initialize optional fields as None
            other_contributors: None,
            version: None,
            number: None,
            publisher: None,
            location: None,
            url: None,
            accessed_date: None,
            additional_info: HashMap::new(),
        })
        .collect();

    Ok(SearchResponse {
        references: results,
    })
}

#[axum::debug_handler]
pub async fn handle_search(
    Json(params): Json<SearchQuery>,
) -> Result<Json<SearchResponse>, (axum::http::StatusCode, String)> {
    match search_pubmed(&params.q).await {
        Ok(results) => Ok(Json(results)),
        Err(e) => {
            let err_str = format!("Failed to search PubMed: {}", e);
            println!("{}", err_str);
            Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, err_str))
        }
    }
}
