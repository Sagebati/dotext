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

use doc;
use doc::{DocumentHandler, HasKind};

pub struct Odp {
    path: Option<PathBuf>,
    data: Cursor<String>,
}

impl HasKind for Odp {
    fn kind(&self) -> &'static str {
        "Open Office Presentation"
    }

    fn ext(&self) -> &'static str {
        "odp"
    }
}

impl DocumentHandler<Odp> for Odp {
    fn open<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut opt = Self::from_reader(File::open(&path)?)?;
        opt.path = Some(path.as_ref().to_path_buf());
        Ok(opt)
    }

    fn from_reader<R: Read + Seek>(r: R) -> io::Result<Odp> {
        let text = doc::open_doc_read_data(r, "content.xml", &["text:p", "text:span"])?;
        Ok(Odp {
            path: None,
            data: Cursor::new(text),
        })
    }
}

impl Read for Odp {
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
        let _ = Odp::open(Path::new("samples/sample.odp"));
    }

    #[test]
    fn read() {
        let mut f = Odp::open(Path::new("samples/sample.odp")).unwrap();

        let mut data = String::new();
        let len = f.read_to_string(&mut data).unwrap();
        println!("len: {}, data: {}", len, data);
    }
}
