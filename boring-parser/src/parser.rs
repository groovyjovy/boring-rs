use crate::boring_structs_110::Boring110;
use crate::error::ParseError;
use quick_xml::Reader;
use quick_xml::de::from_str;
use quick_xml::events::Event;
use quick_xml::se::to_string;
use std::str::{self};

pub trait Parse<T> {
    fn parse_from_str(xml_str: &str) -> Result<T, Box<dyn std::error::Error>>;
}

impl Parse<Boring110> for Boring110 {
    fn parse_from_str(xml_str: &str) -> Result<Boring110, Box<dyn std::error::Error>> {
        let boring: Boring110 = from_str(xml_str)?;
        Ok(boring)
    }
}

pub trait ToXml {
    fn to_xml_string(&self) -> Result<String, Box<dyn std::error::Error>>;
}

impl ToXml for Boring110 {
    fn to_xml_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let xml = to_string(&self)?;
        Ok(xml)
    }
}

pub fn extract_dtd_version(xml: &str) -> Result<String, ParseError> {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) if e.name().as_ref() == "ボーリング情報".as_bytes() => {
                for attr in e.attributes().flatten() {
                    if attr.key.as_ref() == b"DTD_version" {
                        return Ok(attr.unescape_value().unwrap_or_default().to_string());
                    }
                }
                break;
            }
            Ok(Event::Eof) => break,
            _ => {}
        }
        buf.clear();
    }
    Err(ParseError::ParseError(String::from(
        "DTD_version was not found. Plese check your XML file.",
    )))
}
