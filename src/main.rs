use std::env;
use std::fs::File;
use std::io::BufReader;
use xml::EventReader;
use xml::reader::XmlEvent;
use corpus::Corpus;

mod corpus;

fn get_file(args: Vec<String>) -> Option<File> {
    match args.get(1) {
        Some(path) => {
            match File::open(path) {
                Ok(file) => { return Some(file); }
                Err(error) => { println!("filed reading failed. {}", error); }
            }
        }
        None => println!("argument parsing failed"),
    };
    None
}

fn main() {
    if let Some(file) = get_file(env::args().collect()) {
        let reader = EventReader::new(BufReader::new(file));
        let mut corpus = Corpus::new();
        for event in reader {
            match event {
                Ok(event) => {
                    match event {
                        XmlEvent::EndDocument => return,
                        event => corpus.feed(event)
                    }
                }
                Err(error) => {
                    println!("xml parsing failed. {}", error.msg());
                    break;
                }
            }
        }
    }
}
