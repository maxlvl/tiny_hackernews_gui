use colour::{blue, dark_green, red, yellow};
use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Debug)]
pub enum NewsApiError {
    #[error("Api request failed")]
    RequestFailed(ureq::Error),
    #[error("Failed to convert request response into string")]
    RequestStringConversionFailed(std::io::Error),
    #[error("Failed to parse request response")]
    ArticleParseError(serde_json::Error),
}

#[derive(Serialize, Deserialize)]
pub struct Article {
    id: u32,
    title: String,
    time_ago: String,
    comments_count: u16,
    url: String,
}

pub fn get_articles(url: &str) -> Result<Vec<Article>, NewsApiError> {
    let response = ureq::get(url)
        .call()
        .map_err(|e| NewsApiError::RequestFailed(e))?
        .into_string()
        .map_err(|e| NewsApiError::RequestStringConversionFailed(e))?;

    let articles: Vec<Article> =
        serde_json::from_str(&response).map_err(|e| NewsApiError::ArticleParseError(e))?;
    Ok(articles)
}

pub fn render_articles(articles: &Vec<Article>) {
    for a in articles {
        dark_green!("Title:          {}\n", a.title);
        yellow!("URL:            {}\n", a.url);
        red!("Comment Count:  {}\n", a.comments_count);
        blue!("Time Posted:    {}\n\n", a.time_ago);
    }
}
