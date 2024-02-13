use xml::name::QName;
use zip::ZipArchive;

use xml::events::Event;
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

pub struct Pptx {
    path: Option<PathBuf>,
    data: Cursor<String>,
}

impl Pptx {
    pub fn into_txt(self) -> String  {
       self.data.into_inner()
    }

    pub fn txt(&self) -> &str {
        self.data.get_ref()
    }
}

impl HasKind for Pptx {
    fn kind(&self) -> &'static str {
        "Power Point"
    }

    fn ext(&self) -> &'static str {
        "pptx"
    }
}

impl DocumentHandler<Pptx> for Pptx {
    fn from_reader<R: Read + Seek>(reader: R) -> io::Result<Pptx> {
        let mut archive = ZipArchive::new(reader)?;

        let mut xml_data = String::new();

        for i in 0..archive.len() {
            let mut c_file = archive.by_index(i).unwrap();
            if c_file.name().starts_with("ppt/slides") {
                let mut _buff = String::new();
                c_file.read_to_string(&mut _buff);
                xml_data += _buff.as_str();
            }
        }

        let mut txt = Vec::new();

        if xml_data.len() > 0 {
            let mut to_read = false;
            let mut xml_reader = Reader::from_str(xml_data.as_ref());
            loop {
                match xml_reader.read_event() {
                    Ok(Event::Start(ref e)) => match e.name() {
                        QName(b"a:p") => {
                            to_read = true;
                            txt.push("\n".to_string());
                        }
                        QName(b"a:t") => {
                            to_read = true;
                        }
                        _ => (),
                    },
                    Ok(Event::Text(e)) => {
                        if to_read {
                            let text = e.unescape().unwrap().to_string();
                            txt.push(text);
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

        Ok(Pptx {
            path: None,
            data: Cursor::new(txt.join("")),
        })
    }
}

impl Read for Pptx {
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
        let _ = Pptx::open(Path::new("samples/sample.pptx"));
    }

    #[test]
    fn read() {
        let mut f = Pptx::open(Path::new("samples/sample.pptx")).unwrap();

        let mut data = String::new();
        let len = f.read_to_string(&mut data).unwrap();
        println!("len: {}, data: {}", len, data);
    }
}
