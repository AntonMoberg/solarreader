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
    pub date: field::date,
    pub dcterms_publisher: field::publisher,
    pub cover: field::cover,
    pub dcterms_date: field::dcterms_date,
    pub language: field::language,
    pub dcterms_identifier: field::dcterms_identifier,
    pub dcterms_modified: field::dcterms_modified,
    pub creator: field::creator,
    pub dcterms_creator: field::dcterms_creator,
    pub identifier: field::identifier,
    pub dcterms_title: field::dcterms_title,
    pub title: field::title,
    pub rights: field::rights,
    pub dcterms_rights: field::dcterms_rights,
    pub subject: field::subject,
    pub dcterms_language: field::dcterms_language,
    pub dcterms_subject: field::dcterms_subject,
    pub publisher: field::publisher,
    pub rights_holder: field::rights_holder,
}
