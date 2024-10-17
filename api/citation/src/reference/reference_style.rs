use maud::{html, Markup};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Reference {
    authors: Vec<String>,
    year: Option<u16>,
    title: String,
    container: Option<String>, // For journal, book, etc.
    other_contributors: Option<Vec<String>>,
    version: Option<String>,
    number: Option<String>,
    publisher: Option<String>,
    publication_date: Option<String>,
    location: Option<String>,
    pages: Option<String>,
    volume: Option<u32>,
    issue: Option<u32>,
    doi: Option<String>,
    url: Option<String>,
    accessed_date: Option<String>,
    #[serde(flatten)]
    additional_info: HashMap<String, String>, // For any extra fields
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReferenceStyle {
    APA,
    MLA,
    Chicago,
    Harvard,
    Vancouver,
    IEEE,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OutputFormat {
    HTML,
}

impl Reference {
    pub fn format(&self, style: ReferenceStyle, format: OutputFormat) -> String {
        match format {
            OutputFormat::HTML => self.format_html(style).into_string(),
        }
    }
    fn format_html(&self, style: ReferenceStyle) -> Markup {
        match style {
            ReferenceStyle::APA => self.format_apa_html(),
            ReferenceStyle::MLA => self.format_mla_html(),
            ReferenceStyle::Chicago => self.format_chicago_html(),
            ReferenceStyle::Harvard => self.format_harvard_html(),
            ReferenceStyle::Vancouver => self.format_vancouver_html(),
            ReferenceStyle::IEEE => self.format_ieee_html(),
        }
    }

    fn format_apa_html(&self) -> Markup {
        html! {
            p class="apa-reference" {
                @for (index, author) in self.authors.iter().enumerate() {
                    @if index > 0 { ", " }
                    (author)
                }
                @if let Some(year) = self.year {
                    " (" (year) "). "
                }
                i { (self.title) ". " }
                @if let Some(container) = &self.container {
                    em { (container) ", " }
                }
                @if let Some(volume) = self.volume {
                    (volume)
                    @if let Some(issue) = self.issue {
                        "(" (issue) ")"
                    }
                    ", "
                }
                @if let Some(pages) = &self.pages {
                    (pages) ". "
                }
                @if let Some(doi) = &self.doi {
                    "https://doi.org/" (doi)
                }
            }
        }
    }

    fn format_mla_html(&self) -> Markup {
        html! {
            p class="mla-reference" {
                @for (index, author) in self.authors.iter().enumerate() {
                    @if index == 0 {
                        (author) ". "
                    } @else if index < self.authors.len() - 1 {
                        ", " (author)
                    } @else {
                        ", and " (author) ". "
                    }
                }
                i { (self.title) ". " }
                @if let Some(container) = &self.container {
                    em { (container) ", " }
                }
                @if let Some(volume) = self.volume {
                    "vol. " (volume) ", "
                }
                @if let Some(issue) = self.issue {
                    "no. " (issue) ", "
                }
                @if let Some(year) = self.year {
                    (year) ", "
                }
                @if let Some(pages) = &self.pages {
                    "pp. " (pages) ". "
                }
                @if let Some(doi) = &self.doi {
                    "DOI: " (doi)
                }
            }
        }
    }

    fn format_chicago_html(&self) -> Markup {
        html! {
            p class="chicago-reference" {
                @for (index, author) in self.authors.iter().enumerate() {
                    @if index == 0 {
                        (author)
                    } @else if index < self.authors.len() - 1 {
                        ", " (author)
                    } @else {
                        ", and " (author)
                    }
                }
                ". "
                (self.title)
                @if let Some(container) = &self.container {
                    " " i { (container) } " "
                }
                @if let Some(volume) = self.volume {
                    (volume)
                    @if let Some(issue) = self.issue {
                        ", no. " (issue)
                    }
                    " "
                }
                @if let Some(year) = self.year {
                    "(" (year) "): "
                }
                @if let Some(pages) = &self.pages {
                    (pages) ". "
                }
                @if let Some(doi) = &self.doi {
                    "https://doi.org/" (doi)
                }
            }
        }
    }

    fn format_harvard_html(&self) -> Markup {
        html! {
            p class="harvard-reference" {
                @for (index, author) in self.authors.iter().enumerate() {
                    @if index > 0 { ", " }
                    (author)
                }
                @if let Some(year) = self.year {
                    " (" (year) ") "
                }
                (self.title) ","
                @if let Some(container) = &self.container {
                    " " i { (container) } ","
                }
                @if let Some(volume) = self.volume {
                    " " (volume)
                    @if let Some(issue) = self.issue {
                        "(" (issue) ")"
                    }
                    ","
                }
                @if let Some(pages) = &self.pages {
                    " pp. " (pages) "."
                }
                @if let Some(doi) = &self.doi {
                    " DOI: " (doi)
                }
            }
        }
    }

    fn format_vancouver_html(&self) -> Markup {
        html! {
            p class="vancouver-reference" {
                @for (index, author) in self.authors.iter().take(6).enumerate() {
                    @if index > 0 { ", " }
                    (author.split_whitespace().last().unwrap_or(""))
                    " "
                    (author.chars().next().unwrap_or(' '))
                }
                @if self.authors.len() > 6 {
                    ", et al"
                }
                ". "
                (self.title) ". "
                @if let Some(container) = &self.container {
                    i { (container) } ". "
                }
                @if let Some(year) = self.year {
                    (year)
                }
                @if let Some(volume) = self.volume {
                    ";" (volume)
                    @if let Some(issue) = self.issue {
                        "(" (issue) ")"
                    }
                    ":"
                }
                @if let Some(pages) = &self.pages {
                    (pages) ". "
                }
                @if let Some(doi) = &self.doi {
                    "doi: " (doi)
                }
            }
        }
    }

    fn format_ieee_html(&self) -> Markup {
        html! {
            p class="ieee-reference" {
                @for (index, author) in self.authors.iter().enumerate() {
                    @if index > 0 { ", " }
                    (author.split_whitespace().last().unwrap_or(""))
                    " "
                    (author.chars().next().unwrap_or(' '))
                    "."
                }
                (self.title) ","
                @if let Some(container) = &self.container {
                    " " i { (container) } ","
                }
                @if let Some(volume) = self.volume {
                    " vol. " (volume) ","
                }
                @if let Some(issue) = self.issue {
                    " no. " (issue) ","
                }
                @if let Some(pages) = &self.pages {
                    " pp. " (pages) ","
                }
                @if let Some(year) = self.year {
                    " " (year) "."
                }
                @if let Some(doi) = &self.doi {
                    " DOI: " (doi)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_sample_reference() -> Reference {
        Reference {
            authors: vec!["Smith, John".to_string(), "Doe, Jane".to_string()],
            year: Some(2023),
            title: "A Study of Reference Styles".to_string(),
            container: Some("Journal of Citation Studies".to_string()),
            volume: Some(5),
            issue: Some(2),
            pages: Some("123-145".to_string()),
            doi: Some("10.1234/jcs.2023.01".to_string()),
            ..Default::default()
        }
    }

    #[test]
    fn test_format_apa_html() {
        let reference = create_sample_reference();
        let formatted = reference.format(ReferenceStyle::APA, OutputFormat::HTML);

        println!("{}", formatted);
        assert_eq!(
            formatted,
            "<p class=\"apa-reference\"><span class=\"author\">Smith, John</span>, <span class=\"author\">Doe, Jane</span> (<span class=\"year\">2023</span>). <span class=\"title\">A Study of Reference Styles</span>. <span class=\"container\">Journal of Citation Studies</span> vol. <span class=\"volume\">5</span>, no. <span class=\"issue\">2</span>, pp. <span class=\"pages\">123-145</span>. <span class=\"doi\">https://doi.org/10.1234/jcs.2023.01</span></p>"
        );
    }
}
