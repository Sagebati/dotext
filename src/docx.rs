use zip::ZipArchive;

use xml::events::Event;
use xml::name::QName;
use xml::reader::Reader;

use std::clone::Clone;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use zip::read::ZipFile;

use doc::{DocumentHandler, HasKind};
use Odp;

pub struct Docx {
    path: Option<PathBuf>,
    data: Cursor<String>,
}

impl Docx {
    pub fn into_txt(self) -> String  {
       self.data.into_inner()
    }

    pub fn txt(&self) -> &str {
        self.data.get_ref()
    }
}

impl HasKind for Docx {
    fn kind(&self) -> &'static str {
        "Word Document"
    }

    fn ext(&self) -> &'static str {
        "docx"
    }
}

impl DocumentHandler<Docx> for Docx {
    fn from_reader<R: Read + Seek>(read: R) -> io::Result<Docx> {
        let mut archive = ZipArchive::new(read)?;

        let mut xml_data = String::new();

        for i in 0..archive.len() {
            let mut c_file = archive.by_index(i).unwrap();
            if c_file.name() == "word/document.xml" {
                // append the data to our xml_data buffer
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
                    Ok(Event::Start(ref e)) => match e.name() {
                        QName(b"w:p") => {
                            to_read = true;
                            txt.push("\n\n".to_string());
                        }
                        QName(b"w:t") => to_read = true,
                        _ => (),
                    },
                    Ok(Event::Text(e)) => {
                        if to_read {
                            let e = e.unescape().unwrap();
                            txt.push(e.to_string());
                            to_read = false;
                        }
                    }
                    Ok(Event::Eof) => break, // exits the loop when reaching end of file
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

        Ok(Docx {
            path: None,
            //returning a cursor to the txt
            data: Cursor::new(txt.join("")),
        })
    }
}

impl Read for Docx {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.data.read(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};

    #[test]
    fn instantiate() {
        let _ = Docx::open(Path::new("samples/filosofi-logo.docx"));
    }

    #[test]
    fn read() {
        let mut f = Docx::open(Path::new("samples/filosofi-logo.docx")).unwrap();

        let mut data = String::new();
        let len = f.read_to_string(&mut data).unwrap();
        println!("len: {}, data: {}", len, data);
    }
}
