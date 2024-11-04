# Reference Note Processing

This module handles the processing of reference notes in HTML documents, specifically focusing on `<sup>` tags that contain reference numbers.

## Reference Number Format

- Basic format: Single numbers in `<sup>` tags
- Range format: Numbers like "1-5" which expand to ["1", "2", "3", "4", "5"]
- Complex format: Combinations like "1-4, 6, 8-10" are supported

## Validation Process

1. **Pattern Matching**: Since `<sup>` elements can be used for general superscripts, we must validate that the tag actually contains a reference number.

2. **Referent Validation**: Each reference must have a corresponding referent (the content being referenced).
   - Referents typically appear in `<li>` elements within an `<ol>` list
   - Reference IDs are implicitly given by position in the list
   - Container names (e.g., "notes" or "references") help locate referents

## Data Processing

1. **Initial Storage**: Store inner HTML as fallback
2. **JSON Generation**: Convert referent data to JSON for easier reformatting
3. **Database Lookup**: Search reference databases (e.g., PubMed) to extract structured data

## Reference ID Generation

Reference IDs must be:
- Unique
- Stable across program runs
- Preferably meaningful (e.g., PubMed ID when available)

### ID Generation Strategy
1. Use database-specific IDs when available (e.g., PubMed ID)
2. Fall back to MD5 hash of referent string if no specific ID exists
3. Generate UUID as last resort

## Implementation Notes

The `ReferenceUuid` enum provides a comprehensive system for handling various types of academic and scholarly reference identifiers:

```rust
#[derive(Hash)]
pub enum ReferenceUuid {
    Pubmed(String),      // PubMed ID for biomedical literature
    Arxiv(String),       // arXiv.org identifier for preprints
    Doi(String),         // Digital Object Identifier
    Url(String),         // Web URL reference
    HtmlMd5(String),     // MD5 hash of HTML content
    Uuid(uuid::Uuid),    // Fallback UUID
    Isbn(String),        // International Standard Book Number
    Issn(String),        // International Standard Serial Number
    Patent(String),      // Patent reference number
    Orcid(String),       // ORCID researcher identifier
    Handle(String),      // Handle System persistent identifier
    Pmc(String),         // PubMed Central ID
    DataCite(String),    // DataCite DOI for research datasets
    RorId(String),       // Research Organization Registry identifier
    UnknownSource(String), // Fallback for unknown sources
}


impl ReferenceUuid {
    pub fn prefix(&self) -> String {
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
type References = HashMap<ReferenceUuid, Reference>;

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
```
