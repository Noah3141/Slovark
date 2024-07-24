

#[derive(Debug)]
pub enum WikiTextPageInitError {
    MissingCorePiece(String),
    MissingExtraPiece(String),
    NotADictionaryPage(String),
    UnimplementedPOSFound(String),
    NotASubstantiveWord(String),
    InflectedFormPage,
    UndeclinableNoun,
}
pub type InitError = WikiTextPageInitError;


impl WikiTextPageInitError {
    /// Provide 
    pub fn reflect_in(self, score: &mut Score, page: &str) {
        match self {
            WikiTextPageInitError::MissingExtraPiece(msg) => { score.missing_extra_count += 1; }
            WikiTextPageInitError::UnimplementedPOSFound(msg) => {score.pos_error_count += 1; }
            WikiTextPageInitError::NotADictionaryPage(msg) => (),
            WikiTextPageInitError::NotASubstantiveWord(msg) => { score.non_substantive += 1; }
            WikiTextPageInitError::InflectedFormPage => { score.inflected_form_page += 1; }
            WikiTextPageInitError::UndeclinableNoun => { score.undeclinable_noun_page += 1; }
            WikiTextPageInitError::MissingCorePiece(msg) => panic!(
                "\n
=======================================================start\n
{page}
=======================================================end\n\n
{msg}\n\n
{score}
", score = score.readout() 
            ),
        }
    }
}


pub struct Score {
    pub pos_error_count: u32,
    pub missing_extra_count: u32,
    pub inflected_form_page: u32,
    pub undeclinable_noun_page: u32,
    pub non_substantive: u64,
    pub successful: u64,
}
impl Score {
    pub fn new() -> Self {
        Self {
            pos_error_count: 0,
            missing_extra_count: 0,
            inflected_form_page: 0,
            undeclinable_noun_page: 0,
            non_substantive: 0,
            successful: 0,
        }
    }

    pub fn readout(&self) -> String {
        format!(
            "pos_error_count={pos_error_count}
    non_substantive={non_substantive}
    missing_extra_count={missing_extra_count}
    inflected_form_page={inflected_form_page}
    undeclinable_noun_page={undeclinable_noun_page}
    successful={successful}",
            pos_error_count = self.pos_error_count,
            missing_extra_count = self.missing_extra_count,
            successful = self.successful,
            inflected_form_page = self.inflected_form_page,
            undeclinable_noun_page = self.undeclinable_noun_page,
            non_substantive = self.non_substantive
        )
    }
}