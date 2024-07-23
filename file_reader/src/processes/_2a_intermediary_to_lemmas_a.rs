use std::{
    fs::File,
    io::{Read, Write},
};

use crate::RUN_OPTS;

/// Takes an intermediary file and creates a form-lemma key
///
pub fn run() -> Result<(), &'static str> {
    let mut intermediary_text = String::with_capacity(50_000);
    File::options()
        .create(true)
        .write(true)
        .open(RUN_OPTS.intermediary_out())
        .unwrap()
        .read_to_string(&mut intermediary_text)
        .expect("writing to variable to pass out of fn");

    // Parse

    // Create Form-Lemma sheet

    Ok(())
}

fn combine() {
    let mut main_data = String::with_capacity(300_000);
    File::open("C:\\Users\\Noah3\\Code\\Slovark\\lemma_data\\augmented_lemma_key.txt")
        .expect("Open source file")
        .read_to_string(&mut main_data)
        .expect("read to string");

    let mut augmenting_data = String::with_capacity(300_000);
    File::open("C:\\Users\\Noah3\\Code\\Slovark\\lemma_data\\word_database_key.txt")
        .expect("Open source file")
        .read_to_string(&mut augmenting_data)
        .expect("read to string");

    let mut output_file = File::options()
        .append(true)
        .create(true)
        .open("C:\\Users\\Noah3\\Code\\Slovark\\lemma_data\\comprehensive_lemma.csv")
        .expect("output handle");
    output_file
        .write(main_data.as_bytes())
        .expect("Write main data");

    let main_forms: Vec<&str> = main_data
        .split("\n")
        .map(|line| {
            // println!("{line}");
            let comma = line.find(",").expect("find comma delimiter");
            return &line[..comma];
        })
        .collect();

    let new_entries = augmenting_data.split_whitespace();
    // .map(|line| { &line[line.find(",").unwrap()..]});

    output_file.write(b"====").expect("divider");

    for new_entry in new_entries {
        let (new_form, new_lemma) = new_entry.split_once(",").expect("split");
        if !main_forms.contains(&new_form) {
            println!("main forms did not contain {new_form}");
            output_file
                .write(format!("{new_form},{new_lemma}\n").as_bytes())
                .expect("write");
        }
    }
}
