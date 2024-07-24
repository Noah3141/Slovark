use super::{InitError, WikiTextPage};
mod test;

impl WikiTextPage {
    /// 7
    pub fn find_etymology(russian_section: &str) -> Result<Option<String>, InitError> {
        let etymology_start = russian_section.find("===Etymology===");
        match etymology_start {
            None => Ok(None),
            Some(start) => {
                let etymology: &str = &russian_section[start..];
                let start: usize = etymology.find("{{").ok_or(InitError::MissingExtraPiece(
                    "Open bracket to etymology missing!".to_string(),
                ))?;
                let end: usize = etymology.find("}}").ok_or(InitError::MissingExtraPiece(
                    "Closing bracket to etymology missing!".to_string(),
                ))?;
                Ok(Some(etymology[start..end + 2].to_string()))
            }
        }
    }
}