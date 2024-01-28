pub mod field;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MetaError {
    #[error("invalid title: {0}")]
    InvalidTitle(String),
    #[error("invalid date: {0}")]
    InvalidDate(String),
    #[error("date parse error")]
    DateParse(),
    #[error("invalid cover format")]
    InvalidCoverFormat(),
}

#[derive(Debug)]
pub struct Meta {
    pub date: field::Date,
    pub dcterms_publisher: field::Publisher,
    pub cover: field::Cover,
    pub dcterms_date: field::Dcterms_date,
    pub language: field::Language,
    pub dcterms_identifier: field::Dcterms_identifier,
    pub dcterms_modified: field::Dcterms_modified,
    pub creator: field::Creator,
    pub dcterms_creator: field::Dcterms_creator,
    pub identifier: field::Identifier,
    pub dcterms_title: field::Dcterms_title,
    pub title: field::Title,
    pub rights: field::Rights,
    pub dcterms_rights: field::Dcterms_rights,
    pub subject: field::Subject,
    pub dcterms_language: field::Dcterms_language,
    pub dcterms_subject: field::Dcterms_subject,
    pub publisher: field::Publisher,
    pub rights_holder: field::Rights_holder,
}
