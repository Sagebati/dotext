use zip::ZipArchive;

use xml::events::Event;
use xml::name::QName;
use xml::reader::Reader;

use std::clone::Clone;
use std::fs::File;
use std::io;
use std::io::Cursor;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use zip::read::ZipFile;
// this contains common functions and definitions for all formats

pub trait HasKind {
    // kind
    fn kind(&self) -> &'static str;

    // extension
    fn ext(&self) -> &'static str;
}

pub trait DocumentHandler<T>: Read + HasKind {
    fn open<P: AsRef<Path>>(path: P) -> io::Result<T> {
        Self::from_reader(File::open(path)?)
    }

    fn from_bytes(bytes: &[u8]) -> T {
        Self::from_reader(Cursor::new(bytes)).unwrap()
    }

    fn from_reader<R: Read + Seek>(r: R) -> io::Result<T>;
}

// a common function for reading odp, odt, and ods files
pub(crate) fn open_doc_read_data(
    reader: impl Read + Seek,
    content_name: &str,
    tags: &[&str],
) -> io::Result<String> {
    let mut archive = ZipArchive::new(reader)?;

    let mut xml_data = String::new();

    for i in 0..archive.len() {
        let mut c_file = archive.by_index(i).unwrap();
        if c_file.name() == content_name {
            c_file.read_to_string(&mut xml_data);
            break;
        }
    }

    let mut xml_reader = Reader::from_str(xml_data.as_ref());

    let mut txt = Vec::new();

    if xml_data.len() > 0 {
        let mut to_read = false;
        loop {
            match xml_reader.read_event() {
                Ok(Event::Start(ref e)) => {
                    for tag in tags {
                        if e.name() == QName(tag.as_bytes()) {
                            to_read = true;
                            if e.name() == QName(b"text:p") {
                                txt.push("\n\n".to_string());
                            }
                            break;
                        }
                    }
                }
                Ok(Event::Text(e)) => {
                    if to_read {
                        txt.push(e.unescape().unwrap().to_string());
                        to_read = false;
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!(
                            "Error at position {}: {:?}",
                            xml_reader.buffer_position(),
                            e
                        ),
                    ));
                }
                _ => (),
            }
        }
    }

    Ok(txt.join(""))
}
