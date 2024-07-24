pub enum Language {
    Russian,
    Ukrainian,
    Belarusian,
}

impl Language {
    pub fn to_string(self) -> String {
        match self {
            Language::Russian => "Russian".to_string(),
            Language::Ukrainian => "Ukrainian".to_string(),
            Language::Belarusian => "Belarusian".to_string(),
        }
    }
}
