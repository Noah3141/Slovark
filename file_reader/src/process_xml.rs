use std::{fs::File, io::{BufWriter, Write}};

use crate::traits::Searchable;


const PAGE_START_TAG: &[u8; 6]  = b"<page>";
const PAGE_END_TAG: &[u8; 7]  = b"</page>";

// // Open the file
// let file_path = "C:\\Users\\Noah3\\Code\\Slovark\\enwiktionary-20240701-pages-meta-current.xml";
// let file = File::open(file_path).expect("Failed to open file");
// let mut reader = BufReader::new(file);

// // Buffer to hold the data read from the file
// let mut proto_buf: Vec<u8> = vec![0; 1024 * 1024 * 120];
// let mut temp_buffer: Box<[u8]> = proto_buf.into_boxed_slice();

// let rus_out_file = File::options().create(true).write(true).open("./ru/000_page_contents.csv").unwrap();
// let mut rus_out = BufWriter::new(rus_out_file);

// let ukr_out_file = File::options().create(true).write(true).open("./ukr/000_page_contents.csv").unwrap();
// let mut ukr_out = BufWriter::new(ukr_out_file);

// let belr_out_file = File::options().create(true).write(true).open("./belru/000_page_contents.csv").unwrap();
// let mut belr_out = BufWriter::new(belr_out_file);

// println!("Starting...");
// loop {
//     let bytes_read = reader.read(&mut temp_buffer).expect("Failed to read from file");
//     if bytes_read == 0 {
//         break;
//     }
    
//     process_xml::process_buffer(
//         &temp_buffer[..bytes_read], 
//         &mut rus_out,
//         &mut ukr_out,
//         &mut belr_out,
//     );
//     rus_out.flush().expect("Flush");
//     ukr_out.flush().expect("Flush");
//     belr_out.flush().expect("Flush");
// }
// println!("\nComplete!");


pub fn process_buffer(
    buffer: &[u8], 
    rus_out: &mut BufWriter<File>, 
    ukr_out: &mut BufWriter<File>, 
    belr_out: &mut BufWriter<File> 
) {
    let occurrences = buffer.find_all(PAGE_START_TAG);

    println!("- {}", occurrences.len());

    for start_pos in occurrences {
        if let Some(end_pos) = &buffer[start_pos..].find(PAGE_END_TAG) {

            let end_pos = start_pos + end_pos + PAGE_END_TAG.len();
            let page_content = &buffer[start_pos..end_pos];

            if let Some(_) = page_content.find(b"==Russian==") {
                rus_out.write(b"===================================================\n").expect("File write");
                rus_out.write(page_content).expect("File write");
                rus_out.write(b"\n").expect("File write");
            }

            if let Some(_) = page_content.find(b"==Ukrainian==") {
                ukr_out.write(b"===================================================\n").expect("File write");
                ukr_out.write(page_content).expect("File write");
                ukr_out.write(b"\n").expect("File write");
            }

            if let Some(_) = page_content.find(b"==Belarusian==") {
                belr_out.write(b"===================================================\n").expect("File write");
                belr_out.write(page_content).expect("File write");
                belr_out.write(b"\n").expect("File write");
            }
                
        }
    }
}

pub fn extract_title(page_content: &[u8]) -> Option<&[u8]> {
    let title_start_tag = b"<title>";
    let title_end_tag = b"</title>";

    if let Some(start_pos) = page_content.find(title_start_tag) {
        if let Some(end_pos) = page_content[start_pos..].find(title_end_tag) {
            let title_start = start_pos + title_start_tag.len();
            let title_end = start_pos + end_pos;
            let title_bytes = &page_content[title_start..title_end];

            return Some(title_bytes);
        }
    }
    None
}