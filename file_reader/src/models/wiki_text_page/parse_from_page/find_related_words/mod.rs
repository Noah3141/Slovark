use super::{InitError, WikiTextPage};
mod test;

impl WikiTextPage {
    /// 8
    pub fn find_related_terms(russian_section: &str) -> Result<Option<String>, InitError> {
        let related_terms_start = match russian_section.find("====Related terms====") {
            Some(loc) => loc,
            None => return Ok(None),
        };
        let related_terms: &str = &russian_section[related_terms_start..];
        let start: usize = related_terms
            .find("{{")
            .ok_or(InitError::MissingExtraPiece(
                "Open bracket to related terms missing!".to_string(),
            ))?;
        let end: usize = related_terms
            .find("}}")
            .ok_or(InitError::MissingExtraPiece(
                "Closing bracket to related terms missing!".to_string(),
            ))?;
        Ok(Some(related_terms[start..end + 2].to_string()))
    }
}