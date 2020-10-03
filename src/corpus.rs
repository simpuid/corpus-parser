use xml::reader::XmlEvent;

pub struct Corpus {}

impl Corpus {
    pub fn new() -> Corpus {
        Corpus {}
    }

    pub fn feed(&mut self, event: XmlEvent) {
        match event {
            XmlEvent::StartElement { name, .. } => {
                print!("start({}) ", name);
            }
            XmlEvent::EndElement { name, .. } => {
                print!("end({}) ", name);
            }
            _ => ()
        }
    }
}