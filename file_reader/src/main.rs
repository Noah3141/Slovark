#[allow(unused, unused_variables, dead_code)]
use std::fs::File;
use std::{collections::HashMap, io::Read};

use models::WikiTextPage;

mod models;
mod traits;
mod process_xml;

fn main() {
    
    let mut data = String::new();
    File::open("C:\\Users\\Noah3\\Code\\Slovark\\file_reader\\ru\\000_page_contents.csv").expect("Open source file").read_to_string(&mut data).expect("read to string");
    let mut pos_error_count: u16 = 0;
    let mut missing_extra_count: u16 = 0;
    let mut successful: u64 = 0;

    for page in data.split("===================================================") {
        if page == "" { continue }

        let result = WikiTextPage::parse_russian(&page);
        match result {
            Err(init_error) => match init_error {
                models::WikiTextPageInitError::MissingCorePiece(msg) => panic!("
=======================================================start
{page}
=======================================================end

{msg}

pos_error_count={pos_error_count}
missing_extra_count={missing_extra_count}
successful={successful}
"),
                models::WikiTextPageInitError::MissingExtraPiece(msg) => {
                    missing_extra_count += 1;
                    // println!("\n{msg}\n")
                },
                models::WikiTextPageInitError::UnimplementedPOSFound(msg) => {
                    pos_error_count += 1;
                },
                models::WikiTextPageInitError::NotADictionaryPage(msg) => (),
                models::WikiTextPageInitError::NotASubstantiveWord(msg) => (),
                models::WikiTextPageInitError::InflectedFormPage => (),
                models::WikiTextPageInitError::UndeclinableNoun => (),
            },

            Ok(wiki_page) => {
                successful += 1;
                println!("@@@@start\n{}\n@@@@end", wiki_page.table)
            },
        }

    }

    println!("
pos_error_count={pos_error_count}
missing_extra_count={missing_extra_count}
successful={successful}
")

}


