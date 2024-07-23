use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize)]
pub enum PoS {
    Noun,
    Verb,
    Adjective,
    Adverb,
}

impl Display for PoS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PoS::Noun => f.write_str("noun"),
            PoS::Verb => f.write_str("verb"),
            PoS::Adjective => f.write_str("adj"),
            PoS::Adverb => f.write_str("adv"),
        }
    }
}
