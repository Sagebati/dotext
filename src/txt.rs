use zip::ZipArchive;

use xml::events::Event;
use xml::name::QName;
use xml::reader::Reader;

use doc::{DocumentHandler, HasKind};
use std::clone::Clone;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use zip::read::ZipFile;
use Odp;

pub struct Txt {
    path: Option<PathBuf>,
    data: Cursor<String>,
}

impl Txt {
    pub fn into_txt(self) -> String  {
       self.data.into_inner()
    }

    pub fn txt(&self) -> &str {
        self.data.get_ref()
    }
}

impl HasKind for Txt {
    fn kind(&self) -> &'static str {
        "Text File"
    }

    fn ext(&self) -> &'static str {
        "txt"
    }
}

impl DocumentHandler<Txt> for Txt {
    fn from_reader<R: Read + Seek>(mut r: R) -> io::Result<Txt> {
        let mut buff = String::new();
        r.read_to_string(&mut buff)?;

        Ok(Txt {
            path: None,
            data: Cursor::new(buff),
        })
    }
}

impl Read for Txt {
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
        let _ = Txt::open(Path::new("samples/sample.txt"));
    }

    #[test]
    fn read() {
        let mut f = Txt::open(Path::new("samples/sample.txt")).unwrap();

        let mut data = String::new();
        let len = f.read_to_string(&mut data).unwrap();
        println!("len: {}, data: {}", len, data);
    }
}
