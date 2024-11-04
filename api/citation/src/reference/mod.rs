pub mod reference_style;

use serde::{Deserialize, Serialize};

pub trait ReferenceLike: ToString + Serialize + for<'de> Deserialize<'de> {}

impl ReferenceLike for String {}

#[derive(Hash)]
pub enum ReferenceUuid {
    Pubmed(String),        // PubMed ID for biomedical literature
    Arxiv(String),         // arXiv.org identifier for preprints
    Doi(String),           // Digital Object Identifier
    Url(String),           // Web URL reference
    HtmlMd5(String),       // MD5 hash of HTML content
    Uuid(uuid::Uuid),      // Fallback UUID
    Isbn(String),          // International Standard Book Number
    Issn(String),          // International Standard Serial Number
    Patent(String),        // Patent reference number
    Orcid(String),         // ORCID researcher identifier
    Handle(String),        // Handle System persistent identifier
    Pmc(String),           // PubMed Central ID
    DataCite(String),      // DataCite DOI for research datasets
    RorId(String),         // Research Organization Registry identifier
    UnknownSource(String), // Fallback for unknown sources
}

/// Returns the prefix for the reference uuid
/// Prefixes enable us to known where to look for a reference (i.e. pubmed, etc.),
/// and so regenerate the reference in extremis
impl ReferenceUuid {
    pub fn prefix(&self) -> &str {
        match self {
            ReferenceUuid::Pubmed(_) => "pubmed:",
            ReferenceUuid::Arxiv(_) => "arxiv:",
            ReferenceUuid::Doi(_) => "doi:",
            ReferenceUuid::Url(_) => "url:",
            ReferenceUuid::HtmlMd5(_) => "md5:",
            ReferenceUuid::Uuid(_) => "uuid:",
            ReferenceUuid::Isbn(_) => "isbn:",
            ReferenceUuid::Issn(_) => "issn:",
            ReferenceUuid::Patent(_) => "patent:",
            ReferenceUuid::Orcid(_) => "orcid:",
            ReferenceUuid::Handle(_) => "hdl:",
            ReferenceUuid::Pmc(_) => "pmc:",
            ReferenceUuid::DataCite(_) => "datacite:",
            ReferenceUuid::RorId(_) => "ror:",
            ReferenceUuid::UnknownSource(_) => "unknown:",
        }
    }
}

impl From<&str> for ReferenceUuid {
    fn from(s: &str) -> Self {
        // Split into prefix and value parts
        let parts: Vec<&str> = s.splitn(2, ':').collect();
        match parts.as_slice() {
            [prefix, value] => match *prefix {
                "pubmed" => Self::Pubmed(value.to_string()),
                "arxiv" => Self::Arxiv(value.to_string()),
                "doi" => Self::Doi(value.to_string()),
                "url" => Self::Url(value.to_string()),
                "md5" => Self::HtmlMd5(value.to_string()),
                "uuid" => Self::Uuid(uuid::Uuid::parse_str(value).unwrap_or_default()),
                "isbn" => Self::Isbn(value.to_string()),
                "issn" => Self::Issn(value.to_string()),
                "patent" => Self::Patent(value.to_string()),
                "orcid" => Self::Orcid(value.to_string()),
                "hdl" => Self::Handle(value.to_string()),
                "pmc" => Self::Pmc(value.to_string()),
                "datacite" => Self::DataCite(value.to_string()),
                "ror" => Self::RorId(value.to_string()),
                _ => Self::UnknownSource(s.to_string()),
            },
            _ => Self::UnknownSource(s.to_string()),
        }
    }
}

trait Isomorphic<I, O> {
    type Input: From<I>;
    type Output: Into<O>;

    fn transform(input: Self::Input) -> Self::Output;
    fn inverse(output: Self::Output) -> Self::Input;
}
