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
pub struct Txt {
    path: PathBuf,
    data: Cursor<String>,
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
    fn open<P: AsRef<Path>>(path: P) -> io::Result<Txt> {
        let txt = std::fs::read_to_string(path.as_ref())?;

        Ok(Txt {
            path: path.as_ref().to_path_buf(),
            data: Cursor::new(txt),
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
