#![allow(unused_imports, dead_code, unused_must_use)]

extern crate quick_xml as xml;
/**
 * Copyright 2017 Robin Syihab. All rights reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software
 * and associated documentation files (the "Software"), to deal in the Software without restriction,
 * including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all copies
 * or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
 * INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR
 * PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE
 * FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
 * IN THE SOFTWARE.
 *
 */
extern crate zip;

pub mod doc;
pub mod docx;
pub mod odp;
pub mod ods;
pub mod odt;
pub mod pdf;
pub mod pptx;
pub mod txt;
pub mod xlsx;

use std::{
    io::{self, Read},
    path::Path,
};

pub use doc::DocumentHandler;
pub use docx::Docx;
pub use odp::Odp;
pub use ods::Ods;
pub use odt::Odt;
pub use pdf::Pdf;
pub use pptx::Pptx;
pub use txt::Txt;
pub use xlsx::Xlsx;

// map file extension to document handler
pub fn read_text<P: AsRef<Path>>(path: P) -> io::Result<Box<dyn Read>> {
    let path = path.as_ref();
    let ext = path.extension().unwrap().to_str().unwrap();
    match ext {
        "docx" => Ok(Box::new(Docx::open(path)?)),
        "odp" => Ok(Box::new(Odp::open(path)?)),
        "ods" => Ok(Box::new(Ods::open(path)?)),
        "odt" => Ok(Box::new(Odt::open(path)?)),
        "pptx" => Ok(Box::new(Pptx::open(path)?)),
        "txt" => Ok(Box::new(Txt::open(path)?)),
        "pdf" => Ok(Box::new(Pdf::open(path)?)),
        "xlsx" => Ok(Box::new(Xlsx::open(path)?)),
        _ => Err(io::Error::new(
            io::ErrorKind::Other,
            "Unsupported file format",
        )),
    }
}
