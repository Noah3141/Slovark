use std::fs::File;
use std::io::Read;

use crate::models::WikiTextPage;
use crate::RUN_OPTS;
use crate::{constants::INTERMEDIARY_DELIMITER, models};

/// Takes in a Wiki dump pages file with equal sign delimiters, and parses into a list of WikiPage objects
pub fn run() -> Result<(), String> {
    let mut intermediary_text = String::with_capacity(50_000);
    File::options()
        .read(true)
        .open(&RUN_OPTS.intermediary_out())
        .unwrap()
        .read_to_string(&mut intermediary_text);

    // Performance trackers
    let mut pos_error_count: u32 = 0;
    let mut missing_extra_count: u32 = 0;
    let mut inflected_form_page: u32 = 0;
    let mut undeclinable_noun_page: u32 = 0;
    let mut non_substantive: u64 = 0;
    let mut successful: u64 = 0;

    let mut wiki_pages: Vec<WikiTextPage> = Vec::with_capacity(50_000);

    for page in intermediary_text.split(INTERMEDIARY_DELIMITER) {
        if page == "" {
            continue;
        }

        let result = WikiTextPage::parse_russian(&page, false);
        match result {
            Err(init_error) => match init_error {
                models::WikiTextPageInitError::MissingCorePiece(msg) => panic!(
                    "
=======================================================start
{page}
=======================================================end

{msg}

pos_error_count={pos_error_count}
missing_extra_count={missing_extra_count}
successful={successful}

inflected_form_page={inflected_form_page}
undeclinable_noun_page={undeclinable_noun_page}
non_substantive={non_substantive}
"
                ),
                models::WikiTextPageInitError::MissingExtraPiece(msg) => {
                    missing_extra_count += 1;
                    // println!("\n{msg}\n")
                }
                models::WikiTextPageInitError::UnimplementedPOSFound(msg) => {
                    pos_error_count += 1;
                }
                models::WikiTextPageInitError::NotADictionaryPage(msg) => (),
                models::WikiTextPageInitError::NotASubstantiveWord(msg) => {
                    non_substantive += 1;
                }
                models::WikiTextPageInitError::InflectedFormPage => {
                    inflected_form_page += 1;
                    // println!("{:#?}",
                    //     WikiTextPage::parse_russian(&page, true).expect_err("UnimplementedPOSError SHOULD occur reliably")
                    // )
                }
                models::WikiTextPageInitError::UndeclinableNoun => {
                    undeclinable_noun_page += 1;
                }
            },

            Ok(wiki_page) => {
                successful += 1;
                wiki_pages.push(wiki_page);
            }
        }
    }

    println!(
        "Finished 2b: Intermediary to Wikipage! 🗳️
    pos_error_count={pos_error_count}
    non_substantive={non_substantive}
    missing_extra_count={missing_extra_count}
    inflected_form_page={inflected_form_page}
    undeclinable_noun_page={undeclinable_noun_page}
    successful={successful}
    ",
        successful = wiki_pages.len()
    );

    let mut output = File::open(RUN_OPTS.wiki_pages_out()).expect("open output");
    serde_json::to_writer(output, &wiki_pages).expect("writing to json file");

    Ok(())
}
