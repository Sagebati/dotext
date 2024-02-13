extern crate pdf_extract;

use zip::ZipArchive;

use self::pdf_extract::*;
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

pub struct Pdf {
    path: Option<PathBuf>,
    data: Cursor<String>,
}

impl HasKind for Pdf {
    fn kind(&self) -> &'static str {
        "Pdf File"
    }

    fn ext(&self) -> &'static str {
        "pdf"
    }
}

impl DocumentHandler<Pdf> for Pdf {
    fn from_bytes(bytes: &[u8]) -> Pdf {
        Pdf {
            path: None,
            data: Cursor::new(pdf_extract::extract_text_from_mem(&bytes).unwrap()),
        }
    }

    fn from_reader<R: Read>(mut read: R) -> io::Result<Pdf> {
        let mut buff = vec![];

        read.read_to_end(&mut buff);

        Ok(Self::from_bytes(&buff))
    }
}

impl Read for Pdf {
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
        let _ = Pdf::open(Path::new("samples/sample.pdf"));
    }

    #[test]
    fn read() {
        let mut f = Pdf::open(Path::new("samples/sample.pdf")).unwrap();
        let mut data = String::new();
        let len = f.read_to_string(&mut data).unwrap();
        println!("len: {}, data: {}", len, data);
    }
}
